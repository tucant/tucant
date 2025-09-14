-- Your SQL goes here
CREATE TABLE anmeldungen_plan (
    url TEXT NOT NULL,
    name TEXT NOT NULL,
    parent TEXT REFERENCES anmeldungen (url),
    min_cp INT NOT NULL,
    max_cp INT,
    min_modules INT NOT NULL,
    max_modules INT,
    PRIMARY KEY (url)
) STRICT;

CREATE TABLE anmeldungen_entries (
    available_semester TEXT NOT NULL, -- s or w or b
    anmeldung TEXT,
    module_url TEXT NOT NULL,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    credits INT NOT NULL,
    state TEXT NOT NULL, -- not_planned or planned or done
    year INT,
    semester TEXT,
    -- TODO only make id the primary key
    PRIMARY KEY (anmeldung, available_semester, id),
    FOREIGN KEY (anmeldung) REFERENCES anmeldungen (url)
) STRICT;