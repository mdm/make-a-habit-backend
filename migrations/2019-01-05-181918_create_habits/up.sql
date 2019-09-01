-- Your SQL goes here
CREATE TABLE habits (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    start TIMESTAMP NOT NULL,
    time_limit INTEGER NOT NULL,
    done_count INTEGER NOT NULL DEFAULT 0,
    streak_current INTEGER NOT NULL DEFAULT 0,
    streak_max INTEGER NOT NULL DEFAULT 0,
    active BOOLEAN NOT NULL DEFAULT 1
)
