use rocket_contrib::json::Json;
use diesel::prelude::*;

use crate::models::habit::Habit;
use crate::DatabaseConnection;
use crate::schema::habits;

#[get("/")]
pub fn index(db: DatabaseConnection) -> QueryResult<Json<Vec<Habit>>> {
    habits::table.load::<Habit>(&db.0)
        .map(|habit| Json(habit))
}