#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
extern crate dotenv;

pub mod entity;
pub mod schema;

// use diesel::prelude::*;
// use dotenv::dotenv;
// use std::env;

// pub fn establish_connection() -> SqliteConnection {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//
//     SqliteConnection::establish(&database_url)
//         .expect(&format!("Connection Error to {}", database_url))
// }

#[database("sqlite_db")]
pub struct DbConn(diesel::SqliteConnection);
