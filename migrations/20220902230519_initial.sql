-- Add migration script here
CREATE TABLE http_cache (
    url TEXT NOT NULL PRIMARY KEY,
    content TEXT NOT NULL
) STRICT; -- https://www.sqlite.org/stricttables.html