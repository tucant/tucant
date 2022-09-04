PRAGMA foreign_keys = ON;

CREATE TABLE users (
    username TEXT NOT NULL PRIMARY KEY,
    name TEXT, -- TODO not null
    active_session TEXT REFERENCES sessions (session_id)
);

CREATE TABLE sessions (
    session_id TEXT NOT NULL PRIMARY KEY,
    session_nr INTEGER NOT NULL,
    user TEXT NOT NULL REFERENCES users (username)
);

CREATE TABLE http_cache (
    session TEXT NOT NULL REFERENCES sessions (session_id),
    url TEXT NOT NULL PRIMARY KEY,
    content TEXT NOT NULL
);
