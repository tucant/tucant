-- SPDX-FileCopyrightText: The tucant Contributors
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

/*CREATE TEXT SEARCH DICTIONARY english_hunspell (
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
*/
CREATE TABLE semesters (
    id BIGINT NOT NULL,
    name TEXT NOT NULL,
    timestamp_start TIMESTAMP NOT NULL,
    timestamp_end TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE modules_unfinished (
    tucan_id BLOB NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP NOT NULL,
    title TEXT NOT NULL,
    module_id TEXT NOT NULL,
    credits INTEGER NOT NULL,
    content TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE/*,
    tsv tsvector NOT NULL GENERATED ALWAYS AS (
    setweight(to_tsvector('tucan', module_id), 'A') ||
    setweight(to_tsvector('tucan', title), 'A') ||
    setweight(to_tsvector('tucan', content), 'D')) STORED*/
);

-- CREATE INDEX modules_idx ON modules_unfinished USING GIN (tsv);

INSERT INTO modules_unfinished
(tucan_id, tucan_last_checked, title, module_id, credits, content, done) VALUES
('\x4d4080352087108492', current_timestamp, 'TUCANSCHEISS', 'TUCANSCHEISS', 0, 'TUCANSCHEISS', true);

CREATE TABLE module_menu_unfinished (
    tucan_id BLOB NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP NOT NULL,
    name TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    -- there is always only one parent
    parent BLOB REFERENCES module_menu_unfinished (tucan_id)
);

-- CREATE INDEX module_menu_unfinished_parent ON module_menu_unfinished USING HASH (parent);

CREATE TABLE module_menu_module (
    module_menu_id BLOB NOT NULL REFERENCES module_menu_unfinished (tucan_id),
    module_id BLOB NOT NULL REFERENCES modules_unfinished (tucan_id),
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
    user_modules_last_checked TIMESTAMP DEFAULT NULL,
    user_courses_last_checked TIMESTAMP DEFAULT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE sessions (
    matriculation_number INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    session_nr BIGINT NOT NULL,
    session_id TEXT NOT NULL,
    PRIMARY KEY (matriculation_number, session_nr, session_id)
);

CREATE TABLE courses_unfinished (
    tucan_id BLOB NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP NOT NULL,
    title TEXT NOT NULL,
    course_id TEXT NOT NULL,
    sws SMALLINT NOT NULL,
    content TEXT NOT NULL,
    semester BIGINT REFERENCES semesters (id),
    done BOOLEAN NOT NULL DEFAULT FALSE/*,
    tsv tsvector NOT NULL GENERATED ALWAYS AS (
    setweight(to_tsvector('tucan', course_id), 'A') ||
    setweight(to_tsvector('tucan', title), 'A') ||
    setweight(to_tsvector('tucan', content), 'D')) STORED*/
);

-- CREATE INDEX courses_idx ON courses_unfinished USING GIN (tsv);

CREATE TABLE course_groups_unfinished (
    tucan_id BLOB NOT NULL PRIMARY KEY,
    course BLOB NOT NULL REFERENCES courses_unfinished (tucan_id),
    title TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE course_events (
    course BLOB NOT NULL REFERENCES courses_unfinished(tucan_id),
    timestamp_start TIMESTAMP NOT NULL,
    timestamp_end TIMESTAMP NOT NULL,
    room TEXT NOT NULL,
    teachers TEXT NOT NULL,
    PRIMARY KEY (course, timestamp_start, timestamp_end, room)
);

CREATE TABLE course_groups_events (
    course BLOB NOT NULL REFERENCES course_groups_unfinished(tucan_id),
    timestamp_start TIMESTAMP NOT NULL,
    timestamp_end TIMESTAMP NOT NULL,
    room TEXT NOT NULL,
    teachers TEXT NOT NULL,
    PRIMARY KEY (course, timestamp_start, timestamp_end, room)
);

CREATE TABLE module_courses (
    module BLOB NOT NULL REFERENCES modules_unfinished (tucan_id),
    course BLOB NOT NULL REFERENCES courses_unfinished (tucan_id),
    PRIMARY KEY (module, course)
);

CREATE TABLE user_modules (
    user_id INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    module_id BLOB NOT NULL REFERENCES modules_unfinished (tucan_id),
    PRIMARY KEY (user_id, module_id)
);

CREATE TABLE user_courses (
    user_id INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    course_id BLOB NOT NULL REFERENCES courses_unfinished (tucan_id),
    PRIMARY KEY (user_id, course_id)
);

CREATE TABLE user_course_groups (
    user_id INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    course_group_id BLOB NOT NULL REFERENCES course_groups_unfinished (tucan_id),
    PRIMARY KEY (user_id, course_group_id)
);

CREATE TABLE exams_unfinished (
    tucan_id BLOB NOT NULL PRIMARY KEY,
    exam_type TEXT NOT NULL,
    semester BIGINT REFERENCES semesters (id), -- TODO FIXME check whether this is actually the same semester as in the overview
    exam_time_start TIMESTAMP DEFAULT NULL,
    exam_time_end TIMESTAMP DEFAULT NULL,
    registration_start TIMESTAMP NOT NULL,
    registration_end TIMESTAMP NOT NULL,
    unregistration_start TIMESTAMP NOT NULL,
    unregistration_end TIMESTAMP NOT NULL,
    examinator TEXT,
    room TEXT,
    done BOOLEAN NOT NULL DEFAULT FALSE -- it would be nice if this would be a two-value enum and the fields than can be read before are already available and then later the rest is available.
);

CREATE TABLE semester_exams (
    user_id INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    semester BIGINT REFERENCES semesters (id),
    tucan_last_checked TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, semester)
);

CREATE TABLE module_exams (
    module_id BLOB NOT NULL REFERENCES modules_unfinished (tucan_id),
    exam BLOB NOT NULL REFERENCES exams_unfinished (tucan_id),
    PRIMARY KEY (module_id, exam)
);

CREATE TABLE course_exams (
    course_id BLOB NOT NULL REFERENCES courses_unfinished (tucan_id),
    exam BLOB NOT NULL REFERENCES exams_unfinished (tucan_id),
    PRIMARY KEY (course_id, exam)
);

CREATE TABLE user_exams (
    matriculation_number INTEGER NOT NULL REFERENCES users_unfinished (matriculation_number),
    exam BLOB NOT NULL REFERENCES exams_unfinished (tucan_id),
    PRIMARY KEY (matriculation_number, exam)
);

CREATE TABLE vv_menu_unfinished (
    tucan_id TEXT NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP NOT NULL,
    name TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE,
    -- there is always only one parent
    parent TEXT REFERENCES vv_menu_unfinished (tucan_id)
);

CREATE TABLE vv_menu_courses (
    vv_menu_id TEXT NOT NULL REFERENCES vv_menu_unfinished (tucan_id),
    course_id BLOB NOT NULL REFERENCES courses_unfinished (tucan_id),
    PRIMARY KEY (course_id, vv_menu_id)
);

-- the exam types that are associated with the module. for example a "Studienleistung" oder "Fachprüfung"
CREATE TABLE module_exam_types (
    module_id BLOB NOT NULL REFERENCES modules_unfinished (tucan_id),
    exam_type TEXT NOT NULL,
    required BOOLEAN NOT NULL,
    weight SMALLINT NOT NULL,
    PRIMARY KEY(module_id, exam_type)
)