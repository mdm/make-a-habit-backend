use chrono::NaiveDateTime;

use crate::schema::habits;

#[derive(Queryable)] // TODO: split Queryable/Serialize into  different types
pub struct Habit {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub time_limit: i32,
    pub start: NaiveDateTime,
    pub next_due: NaiveDateTime,
    pub done_count: i32,
    pub streak_current: i32,
    pub streak_max: i32,
    pub active: bool,
}

#[derive(Serialize)]
pub struct HabitResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub time_limit: i32,
    pub recurrences: Vec<i32>,
    pub next_due: NaiveDateTime,
    pub done_count: i32,
    pub streak_current: i32,
    pub streak_max: i32,
}

// TODO: separate types for Deserialize (only one version), Insertable and AsChangeset
#[derive(Deserialize)]
pub struct HabitRequest {
    pub name: String,
    pub description: Option<String>,
    pub time_limit: i32,
    pub recurrences: Vec<i32>,
}

#[derive(Insertable)]
#[table_name="habits"]
pub struct NewHabit {
    pub name: String,
    pub description: Option<String>,
    pub time_limit: i32,
    pub start: NaiveDateTime,
    pub next_due: NaiveDateTime,
}

#[derive(AsChangeset)]
#[table_name="habits"]
pub struct ChangedHabit {
    pub name: String,
    pub description: String,
    pub time_limit: i32,
}
