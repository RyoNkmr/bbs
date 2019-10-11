extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use crate::entity::thread::Thread;
use crate::schema::reses;
use base64::encode;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Identifiable, Queryable, Deserialize, Associations)]
#[belongs_to(Thread)]
#[table_name = "reses"]
pub struct Res {
    pub id: i32,
    pub thread_id: i32,
    pub user_name: String,
    pub user_id: String,
    pub email: String,
    pub body: String,
    pub ip: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Deserialize, Associations)]
#[belongs_to(Thread)]
#[table_name = "reses"]
pub struct ResCount {
    pub count: i64,
    pub thread_id: i32,
}

impl Res {
    pub fn to_debug_response(&self) -> DebugResponse {
        DebugResponse {
            id: self.id,
            user_name: self.user_name.clone(),
            user_id: self.user_id.clone(),
            email: self.email.clone(),
            body: self.body.clone(),
            ip: self.ip.to_string(),
            created_at: self.created_at.to_string(),
        }
    }
}

pub struct NewResBuilder {
    thread_id: i32,
    user_name: String,
    user_id: String,
    email: String,
    body: String,
    ip: IpAddr,
    ip_string: String,
    created_at: NaiveDateTime,
}

impl NewResBuilder {
    pub fn new(ip: &IpAddr) -> NewResBuilder {
        let user_id_source = Utc::today().to_string() + &ip.to_string();

        return NewResBuilder {
            thread_id: 0,
            body: String::new(),
            created_at: Utc::now().naive_utc(),
            email: String::new(),
            ip: ip.clone(),
            ip_string: ip.to_string(),
            user_name: String::new(),
            user_id: NewResBuilder::generate_user_id(&user_id_source),
        };
    }

    pub fn user_name(&mut self, user_name: &str) -> &mut NewResBuilder {
        self.user_name = user_name.to_string();
        self
    }

    pub fn email(&mut self, email: &str) -> &mut NewResBuilder {
        self.email = email.to_string();
        self
    }

    pub fn body(&mut self, body: &str) -> &mut NewResBuilder {
        self.body = body.to_string();
        self
    }

    pub fn thread_id(&mut self, thread_id: i32) -> &mut NewResBuilder {
        self.thread_id = thread_id;
        self
    }

    pub fn finalize(&self) -> NewRes {
        NewRes {
            thread_id: self.thread_id,
            user_name: &self.user_name,
            user_id: &self.user_id,
            email: &self.email,
            body: &self.body,
            ip: &self.ip_string,
        }
    }

    fn generate_user_id(source: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(source);
        return encode(&hasher.result_str())[..8].to_string();
    }
}

#[derive(Insertable, Debug)]
#[table_name = "reses"]
pub struct NewRes<'a> {
    pub thread_id: i32,
    pub user_name: &'a str,
    pub user_id: &'a str,
    pub email: &'a str,
    pub body: &'a str,
    pub ip: &'a str,
}

pub struct ResRepository {}

impl ResRepository {
    pub fn post(conn: &SqliteConnection, res: &NewRes) -> Res {
        use crate::schema::reses::dsl::{id, reses};

        diesel::insert_into(reses)
            .values(res)
            .execute(conn)
            .expect("Error on saving new res");

        reses.order(id.desc()).first(conn).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct DebugResponse {
    pub id: i32,
    pub user_name: String,
    pub user_id: String,
    pub email: String,
    pub body: String,
    pub created_at: String,
    pub ip: String,
}
