#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::diesel;
use dotenv::dotenv;

#[database("habits")]
pub struct DatabaseConnection(diesel::SqliteConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(DatabaseConnection::fairing())
        .mount("/", routes![index]).launch();
}
