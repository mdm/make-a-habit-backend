#[derive(Queryable)]
pub struct Recurrence {
    pub id: i32,
    pub recurrence_type: i32,
    pub day_of_week: i32,
    pub day_of_month: i32,
    pub week_of_month: i32,
    pub day_of_year: i32,
    pub week_of_year: i32,
    pub month_of_year: i32,
}
