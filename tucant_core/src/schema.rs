// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    course_events (course, timestamp_start, timestamp_end, room) {
        course -> Binary,
        timestamp_start -> Timestamp,
        timestamp_end -> Timestamp,
        room -> Text,
        teachers -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    course_exams (course_id, exam) {
        course_id -> Binary,
        exam -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    course_groups_events (course, timestamp_start, timestamp_end, room) {
        course -> Binary,
        timestamp_start -> Timestamp,
        timestamp_end -> Timestamp,
        room -> Text,
        teachers -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    course_groups_unfinished (tucan_id) {
        tucan_id -> Binary,
        course -> Binary,
        title -> Text,
        done -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    courses_unfinished (tucan_id) {
        tucan_id -> Binary,
        tucan_last_checked -> Timestamp,
        title -> Text,
        course_id -> Text,
        sws -> SmallInt,
        content -> Text,
        semester -> Nullable<BigInt>,
        done -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    exams_unfinished (tucan_id) {
        tucan_id -> Binary,
        exam_type -> Text,
        semester -> Nullable<BigInt>,
        semester_name -> Nullable<Text>,
        exam_time_start -> Nullable<Timestamp>,
        exam_time_end -> Nullable<Timestamp>,
        registration_start -> Timestamp,
        registration_end -> Timestamp,
        unregistration_start -> Timestamp,
        unregistration_end -> Timestamp,
        examinator -> Nullable<Text>,
        room -> Nullable<Text>,
        done -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    module_courses (module, course) {
        module -> Binary,
        course -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    module_exam_types (module_id, exam_type) {
        module_id -> Binary,
        exam_type -> Text,
        required -> Bool,
        weight -> SmallInt,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    module_exams (module_id, exam) {
        module_id -> Binary,
        exam -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    module_menu_module (module_menu_id, module_id) {
        module_menu_id -> Binary,
        module_id -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    module_menu_unfinished (tucan_id) {
        tucan_id -> Binary,
        tucan_last_checked -> Timestamp,
        name -> Text,
        done -> Bool,
        parent -> Nullable<Binary>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    modules_unfinished (tucan_id) {
        tucan_id -> Binary,
        tucan_last_checked -> Timestamp,
        title -> Text,
        module_id -> Text,
        credits -> Integer,
        content -> Text,
        done -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    semester_exams (user_id, semester) {
        user_id -> Integer,
        semester -> Nullable<BigInt>,
        tucan_last_checked -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    semesters (id) {
        id -> BigInt,
        name -> Text,
        timestamp_start -> Timestamp,
        timestamp_end -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    sessions (matriculation_number, session_nr, session_id) {
        matriculation_number -> Integer,
        session_nr -> BigInt,
        session_id -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    user_course_groups (user_id, course_group_id) {
        user_id -> Integer,
        course_group_id -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    user_courses (user_id, course_id) {
        user_id -> Integer,
        course_id -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    user_exams (matriculation_number, exam) {
        matriculation_number -> Integer,
        exam -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    user_modules (user_id, module_id) {
        user_id -> Integer,
        module_id -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users_unfinished (matriculation_number) {
        matriculation_number -> Integer,
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
        department -> Integer,
        post_title -> Text,
        street -> Text,
        address_addition -> Text,
        country -> Text,
        plz -> Integer,
        city -> Text,
        phone_number -> Text,
        user_modules_last_checked -> Nullable<Timestamp>,
        user_courses_last_checked -> Nullable<Timestamp>,
        done -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    vv_menu_courses (vv_menu_id, course_id) {
        vv_menu_id -> Text,
        course_id -> Binary,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    vv_menu_unfinished (tucan_id) {
        tucan_id -> Text,
        tucan_last_checked -> Timestamp,
        name -> Text,
        done -> Bool,
        parent -> Nullable<Text>,
    }
}

diesel::joinable!(course_events -> courses_unfinished (course));
diesel::joinable!(course_exams -> courses_unfinished (course_id));
diesel::joinable!(course_exams -> exams_unfinished (exam));
diesel::joinable!(course_groups_events -> course_groups_unfinished (course));
diesel::joinable!(course_groups_unfinished -> courses_unfinished (course));
diesel::joinable!(courses_unfinished -> semesters (semester));
diesel::joinable!(exams_unfinished -> semesters (semester));
diesel::joinable!(module_courses -> courses_unfinished (course));
diesel::joinable!(module_courses -> modules_unfinished (module));
diesel::joinable!(module_exam_types -> modules_unfinished (module_id));
diesel::joinable!(module_exams -> exams_unfinished (exam));
diesel::joinable!(module_exams -> modules_unfinished (module_id));
diesel::joinable!(module_menu_module -> module_menu_unfinished (module_menu_id));
diesel::joinable!(module_menu_module -> modules_unfinished (module_id));
diesel::joinable!(semester_exams -> semesters (semester));
diesel::joinable!(semester_exams -> users_unfinished (user_id));
diesel::joinable!(sessions -> users_unfinished (matriculation_number));
diesel::joinable!(user_course_groups -> course_groups_unfinished (course_group_id));
diesel::joinable!(user_course_groups -> users_unfinished (user_id));
diesel::joinable!(user_courses -> courses_unfinished (course_id));
diesel::joinable!(user_courses -> users_unfinished (user_id));
diesel::joinable!(user_exams -> exams_unfinished (exam));
diesel::joinable!(user_exams -> users_unfinished (matriculation_number));
diesel::joinable!(user_modules -> modules_unfinished (module_id));
diesel::joinable!(user_modules -> users_unfinished (user_id));
diesel::joinable!(vv_menu_courses -> courses_unfinished (course_id));
diesel::joinable!(vv_menu_courses -> vv_menu_unfinished (vv_menu_id));

diesel::allow_tables_to_appear_in_same_query!(
    course_events,
    course_exams,
    course_groups_events,
    course_groups_unfinished,
    courses_unfinished,
    exams_unfinished,
    module_courses,
    module_exam_types,
    module_exams,
    module_menu_module,
    module_menu_unfinished,
    modules_unfinished,
    semester_exams,
    semesters,
    sessions,
    user_course_groups,
    user_courses,
    user_exams,
    user_modules,
    users_unfinished,
    vv_menu_courses,
    vv_menu_unfinished,
);
