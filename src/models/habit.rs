#[derive(Queryable, Serialize)]
pub struct Habit {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start: i32,
    pub duration: i32,
    pub done_count: i32,
    pub done_streak: i32,
    pub active: bool,
}
