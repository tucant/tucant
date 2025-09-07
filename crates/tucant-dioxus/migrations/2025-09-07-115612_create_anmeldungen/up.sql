-- Your SQL goes here
CREATE TABLE anmeldungen (
    semester TEXT NOT NULL, -- s or w
    url TEXT NOT NULL,
    name TEXT NOT NULL,
    parent TEXT REFERENCES anmeldungen (url),
    PRIMARY KEY (semester, url)
) STRICT;

CREATE TABLE anmeldungen_entries (
    semester TEXT NOT NULL, -- s or w
    anmeldung TEXT,
    module_url TEXT NOT NULL,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY (anmeldung, module_url),
    FOREIGN KEY (semester, anmeldung) REFERENCES anmeldungen (semester, url)
) STRICT;