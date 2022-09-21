// @generated automatically by Diesel CLI.

diesel::table! {
    module_menu_module (module_menu_id, module_id) {
        module_menu_id -> Array<Int8>,
        module_id -> Int8,
    }
}

diesel::table! {
    module_menu_unfinished (tucan_id) {
        tucan_id -> Array<Int8>,
        tucan_last_checked -> Timestamptz,
        name -> Text,
        normalized_name -> Text,
        parent -> Nullable<Array<Int8>>,
        done -> Bool,
    }
}

diesel::table! {
    modules (tucan_id) {
        tucan_id -> Int8,
        tucan_last_checked -> Timestamptz,
        title -> Text,
        module_id -> Text,
        credits -> Nullable<Int4>,
        content -> Text,
        done -> Bool,
    }
}

diesel::joinable!(module_menu_module -> module_menu_unfinished (module_menu_id));
diesel::joinable!(module_menu_module -> modules (module_id));

diesel::allow_tables_to_appear_in_same_query!(
    module_menu_module,
    module_menu_unfinished,
    modules,
);
