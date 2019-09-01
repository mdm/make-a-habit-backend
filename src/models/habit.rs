use chrono::{NaiveDateTime, Utc};

use crate::schema::habits;

#[derive(Identifiable, Queryable)]
pub struct Habit {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start: NaiveDateTime,
    pub time_limit: i32,
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
    pub start: NaiveDateTime,
    pub time_limit: i32,
    pub recurrences: Vec<i32>,
    pub done_count: i32,
    pub streak_current: i32,
    pub streak_max: i32,
}

impl HabitResponse {
    pub fn new(habit: Habit, recurrences: Vec<i32>) -> HabitResponse {
        HabitResponse {
            id: habit.id,
            name: habit.name,
            description: habit.description,
            start: habit.start,
            time_limit: habit.time_limit,
            recurrences: recurrences,
            done_count: habit.done_count,
            streak_current: habit.streak_current,
            streak_max: habit.streak_max,
        }
    }
}

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
    pub start: NaiveDateTime,
    pub time_limit: i32,
}

impl NewHabit {
    pub fn from_request(habit: HabitRequest) -> NewHabit {
        NewHabit {
            name: habit.name,
            description: habit.description,
            start: Utc::now().naive_local(),
            time_limit: habit.time_limit,
        }
    }
}

#[derive(AsChangeset)]
#[table_name="habits"]
pub struct ChangedHabit {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start: Option<NaiveDateTime>,
    pub time_limit: Option<i32>,
    pub done_count: Option<i32>,
    pub streak_current: Option<i32>,
    pub streak_max: Option<i32>,
}

impl ChangedHabit {
    pub fn from_request(habit: HabitRequest) -> ChangedHabit {
        ChangedHabit {
            name: Some(habit.name),
            description: habit.description,
            start: None,
            time_limit: Some(habit.time_limit),
            done_count: None,
            streak_current: None,
            streak_max: None,
        }
    }

    pub fn from_start(start: NaiveDateTime) -> ChangedHabit {
        ChangedHabit {
            name: None,
            description: None,
            start: Some(start),
            time_limit: None,
            done_count: None,
            streak_current: None,
            streak_max: None,
        }
    }

    pub fn from_statistics(done_count: i32, streak_current: i32, streak_max: i32) -> ChangedHabit {
        ChangedHabit {
            name: None,
            description: None,
            start: None,
            time_limit: None,
            done_count: Some(done_count),
            streak_current: Some(streak_current),
            streak_max: Some(streak_max),
        }
    }
}
