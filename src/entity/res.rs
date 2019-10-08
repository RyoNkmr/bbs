extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use base64::encode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

pub struct Res {
    pub username: String,
    pub id: String,
    pub email: String,
    pub body: String,
    pub ip: IpAddr,
    pub created_at: DateTime<Utc>,
}

impl Res {
    pub fn to_debug_response(&self) -> DebugResponse {
        DebugResponse {
            username: self.username.clone(),
            id: self.id.clone(),
            email: self.email.clone(),
            body: self.body.clone(),
            ip: self.ip.to_string(),
            created_at: self.created_at.to_string(),
        }
    }
}

pub struct ResBuilder {
    username: String,
    id: String,
    email: String,
    body: String,
    ip: IpAddr,
    created_at: DateTime<Utc>,
}

fn generate_id(source: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(source);
    return encode(&hasher.result_str())[..8].to_string();
}

impl ResBuilder {
    pub fn new(ip: IpAddr) -> ResBuilder {
        let id_source = Utc::today().to_string() + &ip.to_string();

        return ResBuilder {
            body: String::new(),
            created_at: Utc::now(),
            email: String::new(),
            id: generate_id(&id_source),
            ip,
            username: String::new(),
        };
    }

    pub fn username(&mut self, username: &str) -> &mut ResBuilder {
        self.username = username.to_string();
        self
    }

    pub fn email(&mut self, email: &str) -> &mut ResBuilder {
        self.email = email.to_string();
        self
    }

    pub fn body(&mut self, body: &str) -> &mut ResBuilder {
        self.body = body.to_string();
        self
    }

    pub fn finalize(&self) -> Res {
        Res {
            username: self.username.clone(),
            id: self.id.clone(),
            email: self.email.clone(),
            body: self.body.clone(),
            ip: self.ip.clone(),
            created_at: self.created_at.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugResponse {
    pub username: String,
    pub id: String,
    pub email: String,
    pub body: String,
    pub created_at: String,
    pub ip: String,
}
