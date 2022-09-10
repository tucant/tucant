PRAGMA foreign_keys = ON;

CREATE TABLE module_menu (
    tucan_id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    normalized_name TEXT NOT NULL,
    parent TEXT REFERENCES module_menu (tucan_id)
);

CREATE TABLE modules (
    tucan_id TEXT NOT NULL PRIMARY KEY,
    tucan_last_checked DATETIME NOT NULL,
    title TEXT NOT NULL,
    module_id TEXT NOT NULL,
    credits INTEGER,
    content TEXT NOT NULL
);

CREATE TABLE module_menu_module (
    module_menu_id TEXT NOT NULL REFERENCES module_menu (tucan_id),
    module_id TEXT NOT NULL REFERENCES modules (tucan_id),
    PRIMARY KEY (module_menu_id, module_id)
);