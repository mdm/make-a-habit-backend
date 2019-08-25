use chrono::{NaiveDateTime, Utc};

use crate::schema::habits;

#[derive(Identifiable, Queryable)]
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

impl HabitResponse {
    pub fn new(habit: Habit, recurrences: Vec<i32>, next_due_option: Option<NaiveDateTime>) -> HabitResponse {
        HabitResponse {
            id: habit.id,
            name: habit.name,
            description: habit.description,
            time_limit: habit.time_limit,
            recurrences: recurrences,
            next_due: match next_due_option {
                Some(next_due) => next_due,
                None => habit.next_due,
            },
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
    pub time_limit: i32,
    pub start: NaiveDateTime,
    pub next_due: Option<NaiveDateTime>,
}

impl NewHabit {
    pub fn from_request(habit: HabitRequest) -> NewHabit {
        NewHabit {
            name: habit.name,
            description: habit.description,
            time_limit: habit.time_limit,
            start: Utc::now().naive_local(),
            next_due: None,
        }
    }
}

#[derive(AsChangeset)]
#[table_name="habits"]
pub struct ChangedHabit {
    pub name: Option<String>,
    pub description: Option<String>,
    pub time_limit: Option<i32>,
    pub next_due: Option<NaiveDateTime>,
}

impl ChangedHabit {
    pub fn from_request(habit: HabitRequest) -> ChangedHabit {
        ChangedHabit {
            name: Some(habit.name),
            description: habit.description,
            time_limit: Some(habit.time_limit),
            next_due: None,
        }
    }

    pub fn from_next_due(next_due: NaiveDateTime) -> ChangedHabit {
        ChangedHabit {
            name: None,
            description: None,
            time_limit: None,
            next_due: Some(next_due),
        }
    }
}
