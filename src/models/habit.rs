use crate::schema::habits;

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

#[derive(Insertable, Deserialize)]
#[table_name="habits"]
pub struct NewHabit {
    pub name: String,
    pub description: Option<String>,
    pub start: i32,
    pub duration: i32,
}

#[derive(AsChangeset, Deserialize)]
#[table_name="habits"]
pub struct ChangedHabit {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start: Option<i32>,
    pub duration: Option<i32>,
    pub active: Option<bool>,
}
