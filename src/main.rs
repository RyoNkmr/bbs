#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

extern crate bbs;

use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::net::SocketAddr;

use bbs::entity::{
    DebugNewThreadResponse, DebugResponse, DebugThreadResponseWithCount, NewResBuilder,
    ResRepository, Thread, ThreadBuilder, ThreadDetail, ThreadRepository,
};
use bbs::form::UserName;
use bbs::DbConn;

/*
   index
*/
#[derive(Serialize)]
struct IndexContext {
    top_threads: Vec<ThreadDetail>,
}

#[get("/")]
fn index(conn: DbConn) -> Template {
    let top_threads = ThreadRepository::get_latest_threads_with_res(&conn);
    let context = IndexContext { top_threads };
    Template::render("index", &context)
}

#[get("/threads")]
fn all_threads(conn: DbConn) -> Json<Vec<DebugThreadResponseWithCount>> {
    let threads = ThreadRepository::get_all_threads_with_count(&conn);
    let mut ret: Vec<DebugThreadResponseWithCount> = Vec::with_capacity(threads.len());
    for (thread, count) in threads.into_iter() {
        ret.push(thread.to_debug_response_with_count(count));
    }
    Json(ret)
}

#[get("/thread/<slug>")]
fn thread(conn: DbConn, slug: String) -> Template {
    let thread = ThreadRepository::get_thread_with_res(&conn, slug);
    Template::render("thread", &thread)
}

#[derive(FromForm)]
struct NewThreadRequest {
    pub title: String,
    pub user_name: UserName,
    pub email: String,
    pub body: String,
}

#[derive(FromForm)]
struct NewResRequest {
    pub user_name: UserName,
    pub email: String,
    pub body: String,
}

#[post("/threads", data = "<req>")]
fn new_thread(
    conn: DbConn,
    req: Form<NewThreadRequest>,
    addr: SocketAddr,
) -> Json<DebugNewThreadResponse> {
    let mut res_builder = NewResBuilder::new(&addr.ip());
    res_builder
        .user_name(&req.user_name.as_str())
        .email(&req.email)
        .body(&req.body);

    Json(
        ThreadBuilder::new(res_builder)
            .title(&req.title)
            .save(&conn),
    )
}

#[post("/thread/<slug>", data = "<req>")]
fn new_res(
    conn: DbConn,
    slug: String,
    req: Form<NewResRequest>,
    addr: SocketAddr,
) -> Json<DebugResponse> {
    let mut builder = NewResBuilder::new(&addr.ip());
    let new_res = builder
        .thread_slug(&slug)
        .user_name(&req.user_name.as_str())
        .email(&req.email)
        .body(&req.body)
        .finalize();

    let res = ResRepository::post(&conn, &new_res);
    Json(res.to_debug_response())
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![index, all_threads, new_res, thread, new_thread],
        )
        .launch();
}
