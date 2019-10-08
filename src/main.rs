#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate bbs;

use chrono::{DateTime, Utc};
use rocket::request::Form;
use rocket::Request;
use rocket_contrib::json::Json;
use std::net::SocketAddr;

use bbs::entity::{DebugResponse, ResBuilder};

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

#[post("/thread/<id>", data = "<req>")]
fn post(id: usize, req: Form<ResRequest>, addr: SocketAddr) -> Json<DebugResponse> {
    let res = ResBuilder::new(addr.ip())
        .username(&req.username)
        .email(&req.email)
        .body(&req.body)
        .finalize();

    return Json(res.to_debug_response());
}

fn main() {
    rocket::ignite().mount("/", routes![index, post]).launch();
}
