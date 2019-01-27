use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use diesel::prelude::*;
use diesel::result::Error;

use crate::models::habit::*;
use crate::DatabaseConnection;
use crate::schema::habits;

#[get("/")]
pub fn index(db: DatabaseConnection) -> Result<Json<Vec<Habit>>, Status> {
    habits::table.load::<Habit>(&db.0)
        .map(|habit| Json(habit))
        .map_err(|error| error_status(error))
}

#[post("/", data = "<habit>")]
pub fn create(habit: Json<NewHabit>, db: DatabaseConnection) -> Result<status::Created<Json<Habit>>, Status> {
    db.0.transaction(|| {
        diesel::insert_into(habits::table)
            .values(&habit.0)
            .execute(&db.0)?;

        habits::table.order(habits::id.desc())
            .first(&db.0)
            .map(|habit: Habit| {
                let url = uri!("/habits", read: id = habit.id).path().to_string();
                let content = Json(habit);
                status::Created(url, Some(content))
            })
    })
    .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn read(id: i32, db: DatabaseConnection) -> Result<Json<Habit>, Status> {
    habits::table.find(id)
        .get_result::<Habit>(&db.0)
        .map(|habit| Json(habit))
        .map_err(|error| error_status(error))
}

#[patch("/<id>", data = "<habit>")]
pub fn update(id: i32, habit: Json<ChangedHabit>, db: DatabaseConnection) -> Result<Json<Habit>, Status> {
    db.0.transaction(|| {
        diesel::update(habits::table.find(id))
            .set(&habit.0)
            .execute(&db.0)?;

        habits::table.find(id)
            .get_result::<Habit>(&db.0)
            .map(|habit| Json(habit))
    })
    .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, db: DatabaseConnection) -> Result<Status, Status> {
    diesel::delete(habits::table.find(id))
        .execute(&db.0)
        .map(|rows_affected| {
            match rows_affected {
                0 => Status::NotFound,
                _ => Status::Ok
            }
        })
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status { // TODO: use for all controllers
    dbg!(&error);
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
