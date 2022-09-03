-- Add migration script here
CREATE TABLE http_cache (
    normalized_url TEXT NOT NULL PRIMARY KEY,
    url TEXT NOT NULL,
    content TEXT NOT NULL
) STRICT; -- https://www.sqlite.org/stricttables.html