extern crate diesel;

use crate::entity::res::{NewResBuilder, Res, ResCount, ResRepository};
use crate::schema::threads;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Identifiable, Queryable, Deserialize)]
#[primary_key(slug)]
#[table_name = "threads"]
pub struct Thread {
    pub slug: String,
    pub title: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Thread {
    pub fn to_debug_response(&self) -> DebugThreadResponse {
        DebugThreadResponse {
            slug: self.slug.clone(),
            title: self.title.clone(),
            updated_at: self.updated_at,
            created_at: self.created_at,
        }
    }
    pub fn to_debug_response_with_count(&self, count: i64) -> DebugThreadResponseWithCount {
        DebugThreadResponseWithCount {
            slug: self.slug.clone(),
            title: self.title.clone(),
            updated_at: self.updated_at,
            created_at: self.created_at,
            count,
        }
    }
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

    pub fn save(&'a mut self, conn: &SqliteConnection) -> DebugNewThreadResponse {
        let timestamp_string = Utc::now().timestamp_millis().to_string();
        let new_thread = NewThread {
            title: &self.title,
            slug: &timestamp_string,
        };
        let thread = ThreadRepository::post(conn, &new_thread);
        let new_res = self.new_res_builder.thread_slug(&thread.slug).finalize();
        let res = ResRepository::post(conn, &new_res);

        DebugNewThreadResponse {
            title: thread.title,
            slug: thread.slug,
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
    pub fn get_all_threads_with_count(conn: &SqliteConnection) -> Vec<(Thread, i64)> {
        use crate::schema::reses::dsl::{reses, thread_slug};
        use crate::schema::threads::dsl::{threads, updated_at as thread_update};
        use diesel::dsl::sql;
        use diesel::types::{BigInt, Text};

        let all_threads = threads
            .order(thread_update.desc())
            .load::<Thread>(conn)
            .expect("Error on getting all threads");

        let thread_slugs = all_threads
            .iter()
            .map(|t| t.slug.as_str())
            .collect::<Vec<&str>>();

        let all_reses_count = reses
            .select(sql::<(Text, BigInt)>(
                "thread_slug, count(thread_slug) as count",
            ))
            .filter(thread_slug.eq_any(thread_slugs))
            .filter(sql("TRUE GROUP BY thread_slug")) // workaround https://github.com/diesel-rs/diesel/issues/210
            .load::<(String, i64)>(conn)
            .expect("Error on getting reses of latest threads");

        let mut count_map = HashMap::new();
        for (slug, count) in all_reses_count.into_iter() {
            count_map.insert(slug, count);
        }

        let mut ret = Vec::with_capacity(all_threads.len());
        for thread in all_threads.into_iter() {
            let count = count_map.get(&thread.slug).unwrap_or(&0);
            ret.push((thread, *count));
        }

        ret
    }

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
        use crate::schema::threads::dsl::{threads, updated_at as thread_update};

        let latest_threads = threads
            .order(thread_update.desc())
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
pub struct DebugNewThreadResponse {
    pub title: String,
    pub slug: String,
    pub user_name: String,
    pub user_id: String,
    pub email: String,
    pub body: String,
    pub created_at: String,
    pub ip: String,
}

#[derive(Debug, Serialize)]
pub struct DebugThreadResponse {
    pub slug: String,
    pub title: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct DebugThreadResponseWithCount {
    pub slug: String,
    pub title: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub count: i64,
}
