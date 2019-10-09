extern crate diesel;

use crate::entity::res::{NewRes, NewResBuilder, Res, ResRepository};
use crate::schema::threads;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Deserialize)]
#[table_name = "threads"]
pub struct Thread {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub struct ThreadBuilder {
    title: String,
    new_res_builder: NewResBuilder,
}

impl<'a> ThreadBuilder {
    pub fn new(new_res_builder: NewResBuilder) -> ThreadBuilder {
        return ThreadBuilder {
            title: String::new(),
            new_res_builder,
        };
    }

    pub fn title(&mut self, title: &str) -> &mut ThreadBuilder {
        self.title = title.to_string();
        self
    }

    pub fn save(&'a mut self, conn: &SqliteConnection) -> DebugThreadResponse {
        let timestamp_string = Utc::now().timestamp().to_string();
        let new_thread = NewThread {
            title: &self.title,
            slug: &timestamp_string,
        };
        let thread = ThreadRepository::post(conn, &new_thread);
        let new_res = self.new_res_builder.thread_id(thread.id).finalize();
        let res = ResRepository::post(conn, &new_res);

        DebugThreadResponse {
            title: thread.title,
            slug: thread.slug,
            thread_id: res.thread_id,
            id: res.id,
            user_name: res.user_name,
            user_id: res.user_id,
            email: res.email,
            body: res.body,
            created_at: res.created_at.to_string(),
            ip: res.ip.clone(),
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name = "threads"]
pub struct NewThread<'a> {
    pub title: &'a str,
    pub slug: &'a str,
}

pub struct ThreadRepository {}

impl ThreadRepository {
    pub fn get_thread_with_res(conn: &SqliteConnection, slug: String) -> (Thread, Vec<Res>) {
        use crate::schema::reses::dsl::created_at as res_cat;
        use crate::schema::threads::dsl::{slug as thread_slug, threads};

        let thread: Thread = threads
            .filter(thread_slug.eq(slug))
            .first(conn)
            .expect("Error on getting thread");

        let all_reses = Res::belonging_to(&thread)
            .order(res_cat.asc())
            .load::<Res>(conn)
            .expect("Error on getting reses of the thread");

        (thread, all_reses)
    }

    pub fn get_latest_threads_with_res(conn: &SqliteConnection) -> Vec<(Thread, Vec<Res>)> {
        use crate::schema::reses::dsl::created_at as res_cat;
        use crate::schema::threads::dsl::{created_at as thread_cat, threads};

        let latest_threads = threads
            .order(thread_cat.desc())
            .limit(20)
            .load::<Thread>(conn)
            .expect("Error on getting latest threads");

        let all_reses = Res::belonging_to(&latest_threads)
            .order(res_cat.asc())
            .load::<Res>(conn)
            .expect("Error on getting reses of latest threads")
            .grouped_by(&latest_threads);

        latest_threads
            .into_iter()
            .zip(all_reses)
            .collect::<Vec<_>>()
    }

    pub fn post<'a>(conn: &SqliteConnection, thread: &NewThread<'a>) -> Thread {
        use crate::schema::threads::dsl::{created_at, threads};

        diesel::insert_into(threads)
            .values(thread)
            .execute(conn)
            .expect("Error on saving new thread");

        threads
            .order(created_at.desc())
            .first(conn)
            .expect("Error on getting new thread after saved")
    }
}

#[derive(Debug, Serialize)]
pub struct DebugThreadResponse {
    pub title: String,
    pub slug: String,
    pub thread_id: i32,
    pub id: i32,
    pub user_name: String,
    pub user_id: String,
    pub email: String,
    pub body: String,
    pub created_at: String,
    pub ip: String,
}
