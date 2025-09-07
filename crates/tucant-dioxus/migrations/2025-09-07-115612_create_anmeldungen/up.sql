-- Your SQL goes here
CREATE TABLE anmeldungen (
    url TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    parent TEXT REFERENCES anmeldungen (url)
);

CREATE TABLE anmeldungen_entries (
    anmeldung TEXT REFERENCES anmeldungen (url),
    module_url TEXT NOT NULL,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY (anmeldung, module_url)
);