use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use diesel::prelude::*;

use crate::models::habit::*;
use crate::DatabaseConnection;
use crate::schema::habits;

#[get("/")]
pub fn index(db: DatabaseConnection) -> QueryResult<Json<Vec<Habit>>> {
    habits::table.load::<Habit>(&db.0)
        .map(|habit| Json(habit))
}

#[post("/", data = "<habit>")]
pub fn create(habit: Json<NewHabit>, db: DatabaseConnection) -> QueryResult<status::Created<Json<Habit>>> {
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
}

#[get("/<id>")]
pub fn read(id: i32, db: DatabaseConnection) -> QueryResult<Json<Habit>> {
    habits::table.find(id)
        .get_result::<Habit>(&db.0)
        .map(|habit| Json(habit))
}

#[put("/<id>", data = "<habit>")]
pub fn update(id: i32, habit: Json<NewHabit>, db: DatabaseConnection) -> QueryResult<Json<Habit>> {
    db.0.transaction(|| {
        diesel::update(habits::table.find(id))
            .set(&habit.0)
            .execute(&db.0)?;

        habits::table.find(id)
            .get_result::<Habit>(&db.0)
            .map(|habit| Json(habit))
    })
}

#[delete("/<id>")]
pub fn delete(id: i32, db: DatabaseConnection) -> QueryResult<Status> {
    diesel::delete(habits::table.find(id))
        .execute(&db.0)
        .map(|_| Status::Ok)
}
