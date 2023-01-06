// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::{Bytea, Text, Timestamptz};
    use diesel_full_text_search::*;

    course_events (course, timestamp_start, timestamp_end, room) {
        course -> Bytea,
        timestamp_start -> Timestamptz,
        timestamp_end -> Timestamptz,
        room -> Text,
        teachers -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::Bytea;
    use diesel_full_text_search::*;

    course_exams (course_id, exam) {
        course_id -> Bytea,
        exam -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::{Bytea, Text, Timestamptz};
    use diesel_full_text_search::*;

    course_groups_events (course, timestamp_start, timestamp_end, room) {
        course -> Bytea,
        timestamp_start -> Timestamptz,
        timestamp_end -> Timestamptz,
        room -> Text,
        teachers -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Bytea, Text};
    use diesel_full_text_search::*;

    course_groups_unfinished (tucan_id) {
        tucan_id -> Bytea,
        course -> Bytea,
        title -> Text,
        done -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Bytea, Int2, Text, Timestamptz};
    use diesel_full_text_search::Tsvector;

    courses_unfinished (tucan_id) {
        tucan_id -> Bytea,
        tucan_last_checked -> Timestamptz,
        title -> Text,
        course_id -> Text,
        sws -> Int2,
        content -> Text,
        done -> Bool,
        tsv -> Tsvector,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Bytea, Nullable, Text, Timestamptz};
    use diesel_full_text_search::*;

    exams_unfinished (tucan_id) {
        tucan_id -> Bytea,
        exam_type -> Text,
        semester -> Text,
        exam_time_start -> Nullable<Timestamptz>,
        exam_time_end -> Nullable<Timestamptz>,
        registration_start -> Timestamptz,
        registration_end -> Timestamptz,
        unregistration_start -> Timestamptz,
        unregistration_end -> Timestamptz,
        examinator -> Nullable<Text>,
        room -> Nullable<Text>,
        done -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::Bytea;
    use diesel_full_text_search::*;

    module_courses (module, course) {
        module -> Bytea,
        course -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::Bytea;
    use diesel_full_text_search::*;

    module_exams (module_id, exam) {
        module_id -> Bytea,
        exam -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::Bytea;
    use diesel_full_text_search::*;

    module_menu_module (module_id, module_menu_id) {
        module_menu_id -> Bytea,
        module_id -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Bytea, Nullable, Text, Timestamptz};
    use diesel_full_text_search::*;

    module_menu_unfinished (tucan_id) {
        tucan_id -> Bytea,
        tucan_last_checked -> Timestamptz,
        name -> Text,
        done -> Bool,
        parent -> Nullable<Bytea>,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Bytea, Int4, Nullable, Text, Timestamptz};
    use diesel_full_text_search::Tsvector;

    modules_unfinished (tucan_id) {
        tucan_id -> Bytea,
        tucan_last_checked -> Timestamptz,
        title -> Text,
        module_id -> Text,
        credits -> Nullable<Int4>,
        content -> Text,
        done -> Bool,
        tsv -> Tsvector,
    }
}

diesel::table! {
    use diesel::sql_types::{Int4, Int8, Text};
    use diesel_full_text_search::*;

    sessions (matriculation_number, session_nr, session_id) {
        matriculation_number -> Int4,
        session_nr -> Int8,
        session_id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::{Bytea, Int4};
    use diesel_full_text_search::*;

    user_course_groups (user_id, course_group_id) {
        user_id -> Int4,
        course_group_id -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::{Bytea, Int4};
    use diesel_full_text_search::*;

    user_courses (user_id, course_id) {
        user_id -> Int4,
        course_id -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::{Bytea, Int4};
    use diesel_full_text_search::*;

    user_exams (matriculation_number, exam) {
        matriculation_number -> Int4,
        exam -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::{Bytea, Int4};
    use diesel_full_text_search::*;

    user_modules (user_id, module_id) {
        user_id -> Int4,
        module_id -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::{Bool, Int4, Nullable, Text, Timestamptz};
    use diesel_full_text_search::*;

    users_unfinished (matriculation_number) {
        matriculation_number -> Int4,
        title -> Text,
        academic_title -> Text,
        post_name -> Text,
        first_name -> Text,
        middle_name -> Text,
        last_name -> Text,
        pre_name -> Text,
        redirect_messages_to_university_email -> Bool,
        subject -> Text,
        email -> Text,
        department -> Int4,
        post_title -> Text,
        street -> Text,
        address_addition -> Text,
        country -> Text,
        plz -> Int4,
        city -> Text,
        phone_number -> Text,
        user_modules_last_checked -> Nullable<Timestamptz>,
        user_courses_last_checked -> Nullable<Timestamptz>,
        user_exams_last_checked -> Nullable<Timestamptz>,
        done -> Bool,
    }
}

diesel::joinable!(course_events -> courses_unfinished (course));
diesel::joinable!(course_exams -> courses_unfinished (course_id));
diesel::joinable!(course_exams -> exams_unfinished (exam));
diesel::joinable!(course_groups_events -> course_groups_unfinished (course));
diesel::joinable!(course_groups_unfinished -> courses_unfinished (course));
diesel::joinable!(module_courses -> courses_unfinished (course));
diesel::joinable!(module_courses -> modules_unfinished (module));
diesel::joinable!(module_exams -> exams_unfinished (exam));
diesel::joinable!(module_exams -> modules_unfinished (module_id));
diesel::joinable!(module_menu_module -> module_menu_unfinished (module_menu_id));
diesel::joinable!(module_menu_module -> modules_unfinished (module_id));
diesel::joinable!(sessions -> users_unfinished (matriculation_number));
diesel::joinable!(user_course_groups -> course_groups_unfinished (course_group_id));
diesel::joinable!(user_course_groups -> users_unfinished (user_id));
diesel::joinable!(user_courses -> courses_unfinished (course_id));
diesel::joinable!(user_courses -> users_unfinished (user_id));
diesel::joinable!(user_exams -> exams_unfinished (exam));
diesel::joinable!(user_exams -> users_unfinished (matriculation_number));
diesel::joinable!(user_modules -> modules_unfinished (module_id));
diesel::joinable!(user_modules -> users_unfinished (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    course_events,
    course_exams,
    course_groups_events,
    course_groups_unfinished,
    courses_unfinished,
    exams_unfinished,
    module_courses,
    module_exams,
    module_menu_module,
    module_menu_unfinished,
    modules_unfinished,
    sessions,
    user_course_groups,
    user_courses,
    user_exams,
    user_modules,
    users_unfinished,
);
