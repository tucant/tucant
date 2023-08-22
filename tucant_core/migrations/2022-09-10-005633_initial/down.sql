-- SPDX-FileCopyrightText: The tucant Contributors
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

DROP TABLE module_exam_types;
DROP TABLE vv_menu_courses;
DROP TABLE vv_menu_unfinished;
DROP TABLE user_exams;
DROP TABLE course_exams;
DROP TABLE module_exams;
DROP TABLE semester_exams;
DROP TABLE exams_unfinished;
DROP TABLE user_course_groups;
DROP TABLE user_courses;
DROP TABLE user_modules;
DROP TABLE module_courses;
DROP TABLE course_groups_events;
DROP TABLE course_events;
DROP TABLE course_groups_unfinished;
--DROP INDEX courses_idx;
DROP TABLE courses_unfinished;
DROP TABLE sessions;
DROP TABLE users_unfinished;
DROP TABLE module_menu_module;
--DROP INDEX module_menu_unfinished_parent;
DROP TABLE module_menu_unfinished;
--DROP INDEX modules_idx;
DROP TABLE modules_unfinished;
DROP TABLE semesters;
DROP TEXT SEARCH CONFIGURATION tucan;
DROP TEXT SEARCH DICTIONARY english_hunspell;
DROP TEXT SEARCH DICTIONARY german_hunspell;