#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
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

#[post("/thread/<id>", data = "<res_request>")]
fn post(id: usize, res_request: Form<ResRequest>, addr: SocketAddr) -> Json<DebugResponse> {
    let today_string = Utc::today().to_string();
    let now = Utc::now();
    let ip = addr.to_string();
    let id_source = today_string + &ip;
    let mut hasher = Sha1::new();
    hasher.input_str(&id_source);

    return Json(DebugResponse {
        id: hasher.result_str(),
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
