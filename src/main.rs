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

use bbs::entity::{
    DebugResponse, DebugThreadResponse, NewResBuilder, ResRepository, ThreadBuilder,
};
use bbs::DbConn;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
struct NewThreadRequest {
    pub title: String,
    pub user_name: String,
    pub email: String,
    pub body: String,
}

#[derive(FromForm)]
struct NewResRequest {
    pub user_name: String,
    pub email: String,
    pub body: String,
}

#[post("/thread", data = "<req>")]
fn new_thread(
    conn: DbConn,
    req: Form<NewThreadRequest>,
    addr: SocketAddr,
) -> Json<DebugThreadResponse> {
    let mut res_builder = NewResBuilder::new(&addr.ip());
    res_builder
        .user_name(&req.user_name)
        .email(&req.email)
        .body(&req.body);
    Json(ThreadBuilder::new(res_builder).save(&conn))
}

#[post("/thread/<thread_id>", data = "<req>")]
fn new_res(
    conn: DbConn,
    thread_id: usize,
    req: Form<NewResRequest>,
    addr: SocketAddr,
) -> Json<DebugResponse> {
    let mut builder = NewResBuilder::new(&addr.ip());
    let new_res = builder
        .thread_id(thread_id as i32)
        .user_name(&req.user_name)
        .email(&req.email)
        .body(&req.body)
        .finalize();

    let res = ResRepository::post(&conn, &new_res);
    Json(res.to_debug_response())
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index, new_res, new_thread])
        .launch();
}
