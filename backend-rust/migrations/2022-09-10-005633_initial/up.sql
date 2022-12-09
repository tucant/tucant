-- SPDX-FileCopyrightText: The tucant Contributors
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

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

CREATE TABLE modules_unfinished (
    tucan_id BYTEA NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    title TEXT NOT NULL,
    module_id TEXT NOT NULL,
    credits INTEGER,
    content TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    tsv tsvector NOT NULL GENERATED ALWAYS AS (
    setweight(to_tsvector('tucan', module_id), 'A') ||
    setweight(to_tsvector('tucan', title), 'A') ||
    setweight(to_tsvector('tucan', content), 'D')) STORED
);

CREATE INDEX modules_idx ON modules_unfinished USING GIN (tsv);

INSERT INTO modules_unfinished
(tucan_id, tucan_last_checked, title, module_id, credits, content, done) VALUES
('\x4d4080352087108492', current_timestamp, 'TUCANSCHEISS', 'TUCANSCHEISS', 0, 'TUCANSCHEISS', true);

CREATE TABLE module_menu_unfinished (
    tucan_id BYTEA NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    name TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    -- there is always only one parent
    parent BYTEA REFERENCES module_menu_unfinished (tucan_id)
);

CREATE INDEX module_menu_unfinished_parent ON module_menu_unfinished USING HASH (parent);

CREATE TABLE module_menu_module (
    module_menu_id BYTEA NOT NULL REFERENCES module_menu_unfinished (tucan_id),
    module_id BYTEA NOT NULL REFERENCES modules_unfinished (tucan_id),
    PRIMARY KEY (module_id, module_menu_id)
);

-- normally we should create a view that removes all that are not done.
-- the problem is diesel doesn't support views. instead we should probably create the proper
-- abstractions on the rust side
CREATE TABLE users_unfinished (
    matriculation_number INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL DEFAULT '',
    academic_title TEXT NOT NULL DEFAULT '',
    post_name TEXT NOT NULL DEFAULT '',
    first_name TEXT NOT NULL DEFAULT '',
    middle_name TEXT NOT NULL DEFAULT '',
    last_name TEXT NOT NULL DEFAULT '',
    pre_name TEXT NOT NULL DEFAULT '',
    redirect_messages_to_university_email BOOLEAN NOT NULL DEFAULT FALSE,
    subject TEXT NOT NULL DEFAULT '',
    email TEXT NOT NULL DEFAULT '',
    department INTEGER NOT NULL DEFAULT 0,
    post_title TEXT NOT NULL DEFAULT '',
    street TEXT NOT NULL DEFAULT '',
    address_addition TEXT NOT NULL DEFAULT '',
    country TEXT NOT NULL DEFAULT '',
    plz INTEGER NOT NULL DEFAULT 0,
    city TEXT NOT NULL DEFAULT '',
    phone_number TEXT NOT NULL DEFAULT '',
    user_modules_last_checked TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    user_courses_last_checked TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE sessions (
    matriculation_number INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    session_nr BIGINT NOT NULL,
    session_id TEXT NOT NULL,
    PRIMARY KEY (matriculation_number, session_nr, session_id)
);

CREATE TABLE courses_unfinished (
    tucan_id BYTEA NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    title TEXT NOT NULL,
    course_id TEXT NOT NULL,
    sws SMALLINT NOT NULL,
    content TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    tsv tsvector NOT NULL GENERATED ALWAYS AS (
    setweight(to_tsvector('tucan', course_id), 'A') ||
    setweight(to_tsvector('tucan', title), 'A') ||
    setweight(to_tsvector('tucan', content), 'D')) STORED
);

CREATE INDEX courses_idx ON courses_unfinished USING GIN (tsv);

CREATE TABLE course_groups_unfinished (
    tucan_id BYTEA NOT NULL PRIMARY KEY,
    course BYTEA NOT NULL REFERENCES courses_unfinished (tucan_id),
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE course_events (
    course BYTEA NOT NULL REFERENCES courses_unfinished(tucan_id),
    timestamp_start TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    timestamp_end TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    room TEXT NOT NULL,
    teachers TEXT NOT NULL,
    PRIMARY KEY (course, timestamp_start, timestamp_end, room)
);

CREATE TABLE course_groups_events (
    course BYTEA NOT NULL REFERENCES course_groups_unfinished(tucan_id),
    timestamp_start TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    timestamp_end TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    room TEXT NOT NULL,
    teachers TEXT NOT NULL,
    PRIMARY KEY (course, timestamp_start, timestamp_end, room)
);

CREATE TABLE module_courses (
    module BYTEA NOT NULL REFERENCES modules_unfinished (tucan_id),
    course BYTEA NOT NULL REFERENCES courses_unfinished (tucan_id),
    PRIMARY KEY (module, course)
);

CREATE TABLE user_modules (
    user_id INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    module_id BYTEA NOT NULL REFERENCES modules_unfinished (tucan_id),
    PRIMARY KEY (user_id, module_id)
);

CREATE TABLE user_courses (
    user_id INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    course_id BYTEA NOT NULL REFERENCES courses_unfinished (tucan_id),
    PRIMARY KEY (user_id, course_id)
);

CREATE TABLE exams_unfinished (
    tucan_id BYTEA NOT NULL PRIMARY KEY,
    exam_type TEXT NOT NULL,
    semester TEXT NOT NULL,
    exam_time_start TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    exam_time_end TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    registration_start TIMESTAMP WITH TIME ZONE NOT NULL,
    registration_end TIMESTAMP WITH TIME ZONE NOT NULL,
    unregistration_start TIMESTAMP WITH TIME ZONE NOT NULL,
    unregistration_end TIMESTAMP WITH TIME ZONE NOT NULL,
    examinator TEXT,
    room TEXT,
    done BOOLEAN NOT NULL DEFAULT FALSE -- it would be nice if this would be a two-value enum and the fields than can be read before are already available and then later the rest is available.
);

CREATE TABLE module_exams (
    module_id BYTEA NOT NULL REFERENCES modules_unfinished (tucan_id),
    exam BYTEA NOT NULL REFERENCES exams_unfinished (tucan_id),
    PRIMARY KEY (module_id, exam)
);

CREATE TABLE course_exams (
    course_id BYTEA NOT NULL REFERENCES courses_unfinished (tucan_id),
    exam BYTEA NOT NULL REFERENCES exams_unfinished (tucan_id),
    PRIMARY KEY (course_id, exam)
);

CREATE TABLE user_exams (
    matriculation_number INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    exam BYTEA NOT NULL REFERENCES exams_unfinished (tucan_id),
    PRIMARY KEY (matriculation_number, exam)
);