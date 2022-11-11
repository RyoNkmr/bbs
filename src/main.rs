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

use bbs::entity::{DebugResponse, NewResponseBuilder, Response, ResponseRepository};
use bbs::form::UserName;
use bbs::DbConn;

/*
   index
*/
#[derive(Serialize)]
struct IndexContext {
    responses: Vec<Response>,
}

const LIMIT: i32 = 20;

#[get("/?<after>")]
fn index(conn: DbConn, after: Option<i32>) -> Template {
    let responses = ResponseRepository::select(&conn, 10, after);
    let context = IndexContext { responses };
    Template::render("index", &context)
}

#[derive(FromForm)]
struct NewResponseRequest {
    pub user_name: UserName,
    pub email: String,
    pub body: String,
}

#[post("/response", data = "<req>")]
fn create_response(
    conn: DbConn,
    req: Form<NewResponseRequest>,
    addr: SocketAddr,
) -> Json<DebugResponse> {
    let mut builder = NewResponseBuilder::new(&addr.ip());
    let new_res = builder
        .user_name(&req.user_name.as_str())
        .email(&req.email)
        .body(&req.body)
        .finalize();

    let res = ResponseRepository::create(&conn, &new_res);
    Json(res.to_debug_response())
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount("/", routes![index, create_response])
        .launch();
}
