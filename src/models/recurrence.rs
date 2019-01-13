#[derive(Queryable)]
pub struct Recurrence {
    pub id: i32,
    pub recurrence_type: i32,
    pub day_of_week: Option<i32>,
    pub day_of_month: Option<i32>,
    pub week_of_month: Option<i32>,
    pub day_of_year: Option<i32>,
    pub week_of_year: Option<i32>,
    pub month_of_year: Option<i32>,
}
