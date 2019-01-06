-- Your SQL goes here
CREATE TABLE recurrences (
    id INTEGER PRIMARY KEY,
    type INTEGER NOT NULL DEFAULT 0,
    day_of_week INTEGER,
    day_of_month INTEGER,
    week_of_month INTEGER,
    day_of_year INTEGER,
    week_of_year INTEGER,
    month_of_year INTEGER
)
