-- SPDX-FileCopyrightText: The tucant Contributors
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

DROP TABLE user_exams;
DROP TABLE course_exams;
DROP TABLE module_exams;
DROP TABLE exams;
DROP TABLE user_courses;
DROP TABLE user_modules;
DROP TABLE module_courses;
DROP TABLE course_groups_events;
DROP TABLE course_events;
DROP TABLE course_groups_unfinished;
DROP INDEX courses_idx;
DROP TABLE courses_unfinished;
DROP TABLE sessions;
DROP TABLE users_unfinished;
DROP TABLE module_menu_module;
DROP INDEX module_menu_unfinished_parent;
DROP TABLE module_menu_unfinished;
DROP INDEX modules_idx;
DROP TABLE modules_unfinished;
DROP TEXT SEARCH CONFIGURATION tucan;
DROP TEXT SEARCH DICTIONARY english_hunspell;
DROP TEXT SEARCH DICTIONARY german_hunspell;