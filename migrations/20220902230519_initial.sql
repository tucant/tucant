PRAGMA foreign_keys = ON;

CREATE TABLE http_cache (
    normalized_url TEXT NOT NULL PRIMARY KEY,
    session_id TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL
);

CREATE TABLE entrypoints (
    entrypoint_url TEXT NOT NULL PRIMARY KEY,
    FOREIGN KEY(entrypoint_url) REFERENCES http_cache(url)
);