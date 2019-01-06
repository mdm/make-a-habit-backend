-- Your SQL goes here
CREATE TABLE habits (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL, 
    description TEXT,
    start INTEGER NOT NULL,
    duration INTEGER NOT NULL,
    done_count INTEGER NOT NULL DEFAULT 0,
    done_streak INTEGER NOT NULL DEFAULT 0,
    active INTEGER NOT NULL DEFAULT 1
)
