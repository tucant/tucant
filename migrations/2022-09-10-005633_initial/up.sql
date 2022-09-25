CREATE TABLE modules_unfinished (
    tucan_id BYTEA NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    title TEXT NOT NULL,
    module_id TEXT NOT NULL,
    credits INTEGER,
    content TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE module_menu_unfinished (
    tucan_id BYTEA NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    name TEXT NOT NULL,
    normalized_name TEXT NOT NULL,
    child_type SMALLINT NOT NULL DEFAULT 0 -- 0 means not done, 1 means menu, 2 means module
);

CREATE TABLE module_menu_tree (
    parent BYTEA NOT NULL REFERENCES module_menu_unfinished (tucan_id),
    child BYTEA NOT NULL REFERENCES module_menu_unfinished (tucan_id),
    PRIMARY KEY (child, parent)
);

CREATE TABLE module_menu_module (
    module_menu_id BYTEA NOT NULL REFERENCES module_menu_unfinished (tucan_id),
    module_id BYTEA NOT NULL REFERENCES modules_unfinished (tucan_id),
    PRIMARY KEY (module_id, module_menu_id)
);

CREATE TABLE users (
    user_id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE users_studies (
    user_id TEXT NOT NULL REFERENCES users (user_id),
    study BYTEA NOT NULL REFERENCES module_menu_unfinished (tucan_id),
    PRIMARY KEY (user_id, study)
);

CREATE TEXT SEARCH DICTIONARY english_hunspell (
    TEMPLATE = ispell,
    DictFile = en_us,
    AffFile = en_us,
    StopWords = english
);

CREATE TEXT SEARCH DICTIONARY german_hunspell (
    TEMPLATE = ispell,
    DictFile = de_de_frami,
    AffFile = de_de_frami,
    StopWords = german
);

CREATE TEXT SEARCH CONFIGURATION tucan (PARSER = default);
ALTER TEXT SEARCH CONFIGURATION tucan ADD MAPPING FOR asciihword, asciiword, hword, hword_asciipart, hword_part, word WITH german_hunspell, english_hunspell, german_stem; -- maybe german_stem but also with english stop words?
ALTER TEXT SEARCH CONFIGURATION tucan ADD MAPPING FOR email, file, float, host, hword_numpart, int, numhword, numword, sfloat, uint, url, url_path, version WITH simple;

CREATE INDEX modules_idx ON modules_unfinished USING GIN (to_tsvector('tucan', content));
