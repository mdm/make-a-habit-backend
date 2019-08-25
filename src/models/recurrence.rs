use crate::models::habit::Habit;
use crate::schema::recurrences;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Habit)]
pub struct Recurrence {
    pub id: i32,
    pub habit_id: i32,
    pub recurrence_type: i32,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub week_of_month: Option<i32>,
    pub day_of_year: Option<i32>,
    pub week_of_year: Option<i32>,
    pub month_of_year: Option<i32>,
}

#[derive(Insertable)]
#[table_name="recurrences"]
pub struct NewRecurrence {
    pub habit_id: i32,
    pub recurrence_type: i32,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub week_of_month: Option<i32>,
    pub day_of_year: Option<i32>,
    pub week_of_year: Option<i32>,
    pub month_of_year: Option<i32>,
}

impl NewRecurrence {
    pub fn new(habit: &Habit, day_of_week: &i32) -> NewRecurrence {
        NewRecurrence {
            habit_id: habit.id,
            recurrence_type: 0,
            day_of_week: Some(*day_of_week),
            day_of_month: None,
            week_of_month: None,
            day_of_year: None,
            week_of_year: None,
            month_of_year: None,
        }
    }
}
