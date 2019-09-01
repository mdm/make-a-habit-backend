-- Your SQL goes here
CREATE TABLE recurrences (
    id INTEGER NOT NULL PRIMARY KEY,
    habit_id INTEGER NOT NULL,
    recurrence_type INTEGER NOT NULL DEFAULT 0,
    day_of_week INTEGER,
    day_of_month INTEGER,
    week_of_month INTEGER,
    day_of_year INTEGER,
    week_of_year INTEGER,
    month_of_year INTEGER,
    FOREIGN KEY (habit_id) REFERENCE habits (id) ON DELETE CASCADE
)
