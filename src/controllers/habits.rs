use std::ops::Add;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use diesel::prelude::*;
use diesel::result::Error;
use chrono::{NaiveDateTime, Utc, Datelike, Duration};


use crate::models::habit::*;
use crate::models::recurrence::*;
use crate::DatabaseConnection;
use crate::schema::habits;

#[get("/")]
pub fn index(db: DatabaseConnection) -> Result<Json<Vec<HabitResponse>>, Status> {
    habits::table.load::<Habit>(&db.0)
        .map(|habits|
            Json(habits.into_iter().map(|habit| {
                let recurrences = fetch_recurrences(&habit, &db);

                HabitResponse::new(habit, recurrences, None)
            }).collect())
        )
        .map_err(|error| error_status(error))
}

#[post("/", data = "<habit_request>")]
pub fn create(habit_request: Json<HabitRequest>, db: DatabaseConnection) -> Result<status::Created<Json<HabitResponse>>, Status> {
    db.0.transaction(|| {
        let recurrences = habit_request.0.recurrences.clone();

        diesel::insert_into(habits::table)
            .values(NewHabit::from_request(habit_request.0))
            .execute(&db.0)?;

        habits::table.order(habits::id.desc())
            .first(&db.0)
            .map(|habit: Habit| {
                create_recurrences(&habit, &recurrences, &db);
                let recurrences = fetch_recurrences(&habit, &db); // TODO: do we need to fetch here?
                let next_due_option = Some(update_next_due(&habit, &db));

                let url = uri!("/habits", read: id = habit.id).path().to_string();
                let content = Json(HabitResponse::new(habit, recurrences, next_due_option));
                status::Created(url, Some(content))
            })
    })
    .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn read(id: i32, db: DatabaseConnection) -> Result<Json<HabitResponse>, Status> {
    habits::table.find(id)
        .get_result::<Habit>(&db.0)
        .map(|habit| {
            let recurrences = fetch_recurrences(&habit, &db);
            Json(HabitResponse::new(habit, recurrences, None))
        })
        .map_err(|error| error_status(error))
}

#[put("/<id>", data = "<habit_request>")]
pub fn update(id: i32, habit_request: Json<HabitRequest>, db: DatabaseConnection) -> Result<Json<HabitResponse>, Status> {
    db.0.transaction(|| {
        let recurrences = habit_request.0.recurrences.clone();

        diesel::update(habits::table.find(id))
            .set(ChangedHabit::from_request(habit_request.0))
            .execute(&db.0)?;

        habits::table.find(id)
            .get_result::<Habit>(&db.0)
            .map(|habit| {
                create_recurrences(&habit, &recurrences, &db);
                let recurrences = fetch_recurrences(&habit, &db); // TODO: do we need to fetch here?
                let next_due_option = Some(update_next_due(&habit, &db));
  
                Json(HabitResponse::new(habit, recurrences, next_due_option))
            })
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

#[post("/<id>/done")]
pub fn mark_done(id: i32, db: DatabaseConnection) -> Result<Json<HabitResponse>, Status> {
    habits::table.find(id)
        .get_result::<Habit>(&db.0)
        .map(|habit| {
            let recurrences = fetch_recurrences(&habit, &db);
            let next_due_option = Some(update_next_due(&habit, &db));

            Json(HabitResponse::new(habit, recurrences, next_due_option))
        })
    .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

// TODO: improve error handling
fn fetch_recurrences(habit: &Habit, db: &DatabaseConnection) -> Vec<i32> {
    match Recurrence::belonging_to(habit).load::<Recurrence>(&db.0) {
        Ok(recurrences) => recurrences.iter()
            .filter(|recurrence|
                recurrence.recurrence_type == 0
            ).filter_map(|recurrence|
                recurrence.day_of_week
            ).collect(),
        Err(_err) => Vec::new(),
    }
}

// TODO: improve error handling
fn create_recurrences(habit: &Habit, recurrences: &Vec<i32>, db: &DatabaseConnection) -> Vec<Recurrence> {
    Vec::new()
}

// TODO: improve error handling
fn update_next_due(habit: &Habit, db: &DatabaseConnection) -> NaiveDateTime {
    let now = Utc::now().naive_local();

    let recurrences = fetch_recurrences(habit, db);
    let day_of_week = now.date().weekday().num_days_from_monday();
    let next_due_in_days = recurrences.into_iter()
        .map(|recurrence| if recurrence <= day_of_week as i32 {
            recurrence + 7 - day_of_week as i32 + 1
        } else {
            recurrence - day_of_week as i32 + 1
        }).min().unwrap();

    let next_due = now.add(Duration::days(next_due_in_days as i64)).date().and_hms(0, 0, 0);
    let changed_habit = ChangedHabit::from_next_due(next_due);

    diesel::update(habits::table.find(habit.id))
        .set(&changed_habit)
        .execute(&db.0).unwrap();

    changed_habit.next_due.unwrap()
}