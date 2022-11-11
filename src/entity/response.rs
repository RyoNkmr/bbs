extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use crate::schema::responses;
use base64::encode;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "responses"]
pub struct Response {
    pub id: i32,
    pub user_name: String,
    pub user_id: String,
    pub email: String,
    pub body: String,
    pub ip: String,
    pub created_at: NaiveDateTime,
}

impl Response {
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

pub struct NewResponseBuilder {
    user_name: String,
    user_id: String,
    email: String,
    body: String,
    ip: IpAddr,
    ip_string: String,
    created_at: NaiveDateTime,
}

impl NewResponseBuilder {
    pub fn new(ip: &IpAddr) -> NewResponseBuilder {
        let user_id_source = Utc::today().to_string() + &ip.to_string();

        return NewResponseBuilder {
            body: String::new(),
            created_at: Utc::now().naive_utc(),
            email: String::new(),
            ip: ip.clone(),
            ip_string: ip.to_string(),
            user_name: String::new(),
            user_id: NewResponseBuilder::generate_user_id(&user_id_source),
        };
    }

    pub fn user_name(&mut self, user_name: &str) -> &mut NewResponseBuilder {
        self.user_name = user_name.to_string();
        self
    }

    pub fn email(&mut self, email: &str) -> &mut NewResponseBuilder {
        self.email = email.to_string();
        self
    }

    pub fn body(&mut self, body: &str) -> &mut NewResponseBuilder {
        self.body = body.to_string();
        self
    }

    pub fn finalize(&self) -> NewResponse {
        NewResponse {
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
#[table_name = "responses"]
pub struct NewResponse<'a> {
    pub user_name: &'a str,
    pub user_id: &'a str,
    pub email: &'a str,
    pub body: &'a str,
    pub ip: &'a str,
}

pub struct ResponseRepository {}

impl ResponseRepository {
    pub fn create(conn: &SqliteConnection, res: &NewResponse) -> Response {
        use crate::schema::responses::dsl::{id, responses};

        diesel::insert_into(responses)
            .values(res)
            .execute(conn)
            .expect("Error on saving new res");

        responses.order(id.desc()).first(conn).unwrap()
    }

    pub fn select(conn: &SqliteConnection, limit: i64, after_id: Option<i32>) -> Vec<Response> {
        use crate::schema::responses::dsl::{id, responses};

        let result = match after_id {
            Some(ai) => responses
                .order(id.desc())
                .filter(id.lt(ai))
                .limit(limit)
                .load::<Response>(conn),
            None => responses
                .order(id.desc())
                .limit(limit)
                .load::<Response>(conn),
        };

        result.expect("failed to select responses")
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
