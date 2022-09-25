// @generated automatically by Diesel CLI.

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
    modules_unfinished (tucan_id) {
        tucan_id -> Bytea,
        tucan_last_checked -> Timestamptz,
        title -> Text,
        module_id -> Text,
        credits -> Nullable<Int4>,
        content -> Text,
        done -> Bool,
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

diesel::joinable!(module_menu_module -> module_menu_unfinished (module_menu_id));
diesel::joinable!(module_menu_module -> modules_unfinished (module_id));
diesel::joinable!(users_studies -> module_menu_unfinished (study));
diesel::joinable!(users_studies -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    module_menu_module,
    module_menu_tree,
    module_menu_unfinished,
    modules_unfinished,
    users,
    users_studies,
);
