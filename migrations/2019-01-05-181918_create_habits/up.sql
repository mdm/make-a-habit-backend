-- Your SQL goes here
CREATE TABLE habits (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    time_limit INTEGER NOT NULL,
    start TIMESTAMP NOT NULL,
    next_due TIMESTAMP NOT NULL,
    done_count INTEGER NOT NULL DEFAULT 0,
    streak_current INTEGER NOT NULL DEFAULT 0,
    streak_max INTEGER NOT NULL DEFAULT 0,
    active BOOLEAN NOT NULL DEFAULT 1
)
