#![allow(proc_macro_derive_resolution_fallback)] // TODO: remove this when diesel handles it properly
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod schema;
mod controllers;
mod models;
mod errors;

// use rocket_contrib::databases::diesel;
use dotenv::dotenv;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[database("habits")]
pub struct DatabaseConnection(diesel::SqliteConnection);

fn main() {
    dotenv().ok();

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::All,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    rocket::ignite()
        .attach(cors)
        .attach(DatabaseConnection::fairing())
        .mount("/habits", routes![
            controllers::habits::index,
            controllers::habits::create,
            controllers::habits::read,
            controllers::habits::update,
            controllers::habits::delete,
            controllers::habits::mark_done,
        ])
        .register(catchers![errors::internal_server_error])
        .launch();
}
