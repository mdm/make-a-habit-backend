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

                HabitResponse::new(habit, recurrences)
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
                let now = Utc::now().naive_local();
                let start = update_start(&habit, &recurrences, &now, &db);

                let url = uri!("/habits", read: id = habit.id).path().to_string();
                let mut response = HabitResponse::new(habit, recurrences);
                response.start = start;
                let content = Json(response);
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
            Json(HabitResponse::new(habit, recurrences))
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
                let now = Utc::now().naive_local();

                if habit.start > now {
                    let start = update_start(&habit, &recurrences, &now, &db);
                    let mut response = HabitResponse::new(habit, recurrences);
                    response.start = start;
                    Json(response)
                } else {
                    Json(HabitResponse::new(habit, recurrences))
                }
            })
    })
    .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, db: DatabaseConnection) -> Result<Status, Status> {
    use crate::schema::recurrences;

    db.0.transaction(|| {
        diesel::delete(recurrences::table.filter(recurrences::habit_id.eq(id)))
            .execute(&db.0)?;

        diesel::delete(habits::table.find(id))
            .execute(&db.0)
            .map(|rows_affected| {
                match rows_affected {
                    0 => Status::NotFound, // TODO: roll back transaction
                    _ => Status::Ok
                }
            })
    })
    .map_err(|error| error_status(error))
}

#[post("/<id>/done")]
pub fn mark_done(id: i32, db: DatabaseConnection) -> Result<Json<HabitResponse>, Status> {
    habits::table.find(id)
        .get_result::<Habit>(&db.0)
        .map(|habit| {
            let recurrences = fetch_recurrences(&habit, &db);
            let now = Utc::now().naive_local();
            let start = update_start(&habit, &recurrences, &now, &db);
            let (done_count, streak_current, streak_max) = update_statistics(&habit, &now, &db);

            let mut response = HabitResponse::new(habit, recurrences);
            response.start = start;
            response.done_count = done_count;
            response.streak_current = streak_current;
            response.streak_max = streak_max;

            Json(response)
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
fn create_recurrences(habit: &Habit, days_of_week: &Vec<i32>, db: &DatabaseConnection) {
    use crate::schema::recurrences;

    db.0.transaction(|| {
        diesel::delete(recurrences::table.filter(recurrences::habit_id.eq(habit.id)))
            .execute(&db.0)?;

        days_of_week.iter()
            .map(|day_of_week| {
                diesel::insert_into(recurrences::table)
                    .values(&NewRecurrence::new(&habit, day_of_week))
                    .execute(&db.0)
            })
            .last().unwrap()
    }).unwrap();
}

// TODO: improve error handling
fn update_start(habit: &Habit, recurrences: &Vec<i32>, now: &NaiveDateTime, db: &DatabaseConnection) -> NaiveDateTime {
    let day_of_week = now.date().weekday().num_days_from_monday();
    let start_in_days = recurrences.iter()
        .map(|recurrence| if recurrence <= &(day_of_week as i32) {
            recurrence + 7 - day_of_week as i32
        } else {
            recurrence - day_of_week as i32
        }).min().unwrap();

    let start = now.add(Duration::days(start_in_days as i64)).date().and_hms(0, 0, 0);

    let changed_habit = ChangedHabit::from_start(start);

    diesel::update(habits::table.find(habit.id))
        .set(&changed_habit)
        .execute(&db.0).unwrap();

    start
}

fn update_statistics(habit: &Habit, now: &NaiveDateTime, db: &DatabaseConnection) -> (i32, i32, i32) {
    let done_count = habit.done_count + 1;

    let streak_current = if now < &habit.start.add(Duration::days(habit.time_limit as i64)).date().and_hms(0, 0, 0) {
        habit.streak_current + 1
    } else {
        1
    };

    let streak_max = if streak_current > habit.streak_max {
        streak_current
    } else {
        habit.streak_max
    };

    let changed_habit = ChangedHabit::from_statistics(done_count, streak_current, streak_max);

    diesel::update(habits::table.find(habit.id))
        .set(&changed_habit)
        .execute(&db.0).unwrap();

    (done_count, streak_current, streak_max)
}
