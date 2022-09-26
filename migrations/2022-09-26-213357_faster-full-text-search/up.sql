DROP INDEX IF EXISTS courses_idx;

ALTER TABLE courses_unfinished ADD COLUMN tsv tsvector NOT NULL GENERATED ALWAYS AS (
    setweight(to_tsvector('tucan', course_id), 'A') ||
    setweight(to_tsvector('tucan', title), 'A') ||
    setweight(to_tsvector('tucan', content), 'D')) STORED;

CREATE INDEX courses_idx ON courses_unfinished USING GIN (tsv);

DROP INDEX IF EXISTS modules_idx;

ALTER TABLE modules_unfinished ADD COLUMN tsv tsvector NOT NULL GENERATED ALWAYS AS (
    setweight(to_tsvector('tucan', module_id), 'A') ||
    setweight(to_tsvector('tucan', title), 'A') ||
    setweight(to_tsvector('tucan', content), 'D')) STORED;

CREATE INDEX modules_idx ON modules_unfinished USING GIN (tsv);