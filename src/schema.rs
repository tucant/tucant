// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tsvector", schema = "pg_catalog"))]
    pub struct Tsvector;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tsvector;

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
    module_courses (module, course) {
        module -> Bytea,
        course -> Bytea,
    }
}

diesel::table! {
    module_menu_module (module_id, module_menu_id) {
        module_menu_id -> Bytea,
        module_id -> Bytea,
    }
}

diesel::table! {
    module_menu_tree (child, parent) {
        parent -> Bytea,
        child -> Bytea,
    }
}

diesel::table! {
    module_menu_unfinished (tucan_id) {
        tucan_id -> Bytea,
        tucan_last_checked -> Timestamptz,
        name -> Text,
        normalized_name -> Text,
        child_type -> Int2,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tsvector;

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
    users (user_id) {
        user_id -> Text,
        name -> Text,
    }
}

diesel::table! {
    users_studies (user_id, study) {
        user_id -> Text,
        study -> Bytea,
    }
}

diesel::joinable!(module_courses -> courses_unfinished (course));
diesel::joinable!(module_courses -> modules_unfinished (module));
diesel::joinable!(module_menu_module -> module_menu_unfinished (module_menu_id));
diesel::joinable!(module_menu_module -> modules_unfinished (module_id));
diesel::joinable!(users_studies -> module_menu_unfinished (study));
diesel::joinable!(users_studies -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    courses_unfinished,
    module_courses,
    module_menu_module,
    module_menu_tree,
    module_menu_unfinished,
    modules_unfinished,
    users,
    users_studies,
);
