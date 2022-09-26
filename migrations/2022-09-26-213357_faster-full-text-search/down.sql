DROP INDEX IF EXISTS courses_idx;
DROP INDEX IF EXISTS modules_idx;

ALTER TABLE courses_unfinished DROP COLUMN tsv;
ALTER TABLE modules_unfinished DROP COLUMN tsv;