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

CREATE TABLE module_menu (
    username TEXT NOT NULL REFERENCES users (username),
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    normalized_name TEXT NOT NULL,
    parent INTEGER REFERENCES module_menu (id)
);

CREATE UNIQUE INDEX module_menu_idx ON module_menu (username, IFNULL(parent, 0), name);

-- TODO FIXME username in key
CREATE TABLE modules (
    username TEXT NOT NULL REFERENCES users (username),
    title TEXT NOT NULL,
    module_id TEXT NOT NULL PRIMARY KEY,
    shortcode TEXT NOT NULL,
    credits INTEGER,
    responsible_person TEXT NOT NULL,
    content TEXT NOT NULL
);

-- TODO FIXME add username
CREATE TABLE module_menu_module (
    module_menu_id INTEGER NOT NULL REFERENCES module_menu (id),
    module_id TEXT NOT NULL REFERENCES modules (module_id),
    PRIMARY KEY (module_menu_id, module_id)
);