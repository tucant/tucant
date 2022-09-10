
CREATE TABLE modules (
    tucan_id TEXT NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    title TEXT NOT NULL,
    module_id TEXT NOT NULL,
    credits INTEGER,
    content TEXT NOT NULL
);

-- TODO FIXME maybe this shows that the parent is not actually unique because
-- some menu parts are integrated at different places
-- if that's the case we need to fix the definition here
CREATE TABLE module_menu (
    tucan_id TEXT NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    name TEXT NOT NULL,
    normalized_name TEXT NOT NULL,
    parent TEXT REFERENCES module_menu (tucan_id)
);

-- Looking at the comment above maybe this is not necessary
CREATE TABLE module_menu_module (
    module_menu_id TEXT NOT NULL REFERENCES module_menu (tucan_id),
    module_id TEXT NOT NULL REFERENCES modules (tucan_id),
    PRIMARY KEY (module_menu_id, module_id)
);