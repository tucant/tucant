-- SPDX-FileCopyrightText: The tucant Contributors
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

DROP INDEX courses_idx;
DROP INDEX modules_idx;

DROP TABLE course_groups_events;
DROP TABLE course_events;
DROP TABLE course_groups_unfinished;

DROP TABLE module_courses;
DROP TABLE courses_unfinished;
DROP TABLE users_studies;
DROP TABLE users;
DROP TABLE module_menu_module;
DROP TABLE module_menu_unfinished;
DROP TABLE modules_unfinished;

DROP TEXT SEARCH CONFIGURATION tucan;
DROP TEXT SEARCH DICTIONARY english_hunspell;
DROP TEXT SEARCH DICTIONARY german_hunspell;