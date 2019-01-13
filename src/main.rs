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

#[database("habits")]
pub struct DatabaseConnection(diesel::SqliteConnection);

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(DatabaseConnection::fairing())
        .mount("/habits", routes![
            controllers::habits::index,
            controllers::habits::create,
            controllers::habits::read,
            controllers::habits::update,
            controllers::habits::delete,
        ])
        .register(catchers![errors::internal_server_error])
        .launch();
}
