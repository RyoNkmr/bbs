#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate base64;
extern crate chrono;
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use base64::{decode, encode};
use chrono::{DateTime, Utc};
use rocket::request::Form;
use rocket::Request;
use rocket_contrib::json::Json;
use std::net::SocketAddr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
struct ResRequest {
    pub username: String,
    pub email: String,
    pub body: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DebugResponse {
    pub username: String,
    pub id: String,
    pub email: String,
    pub body: String,
    pub ip: String,
    pub created_at: String,
}

fn generate_id(source: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(source);
    return encode(&hasher.result_str())[..8].to_string();
}

#[post("/thread/<id>", data = "<res_request>")]
fn post(id: usize, res_request: Form<ResRequest>, addr: SocketAddr) -> Json<DebugResponse> {
    let today_string = Utc::today().to_string();
    let now = Utc::now();
    let ip = addr.ip().to_string();
    let id_source = today_string + &ip;

    return Json(DebugResponse {
        id: generate_id(&id_source).to_string(),
        ip,
        username: res_request.username.clone(),
        email: res_request.email.clone(),
        body: res_request.body.clone(),
        created_at: now.to_string(),
    });
}

fn main() {
    rocket::ignite().mount("/", routes![index, post]).launch();
}
