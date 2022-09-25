CREATE TABLE courses_unfinished (
    tucan_id BYTEA NOT NULL PRIMARY KEY,
    tucan_last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    title TEXT NOT NULL,
    course_id TEXT NOT NULL,
    sws SMALLINT NOT NULL,
    content TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE module_courses (
    module BYTEA NOT NULL REFERENCES modules_unfinished (tucan_id),
    course BYTEA NOT NULL REFERENCES courses_unfinished (tucan_id),
    PRIMARY KEY (module, course)
);

CREATE INDEX courses_idx ON courses_unfinished USING GIN ((
    setweight(to_tsvector('tucan', course_id), 'A') ||
    setweight(to_tsvector('tucan', title), 'A') ||
    setweight(to_tsvector('tucan', content), 'D')
));