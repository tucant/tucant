// @generated automatically by Diesel CLI.

diesel::table! {
    module_menu (tucan_id) {
        tucan_id -> Array<Nullable<Int4>>,
        tucan_last_checked -> Timestamptz,
        name -> Text,
        normalized_name -> Text,
        parent -> Nullable<Array<Nullable<Int4>>>,
    }
}

diesel::table! {
    module_menu_module (module_menu_id, module_id) {
        module_menu_id -> Array<Nullable<Int4>>,
        module_id -> Int4,
    }
}

diesel::table! {
    modules (tucan_id) {
        tucan_id -> Int4,
        tucan_last_checked -> Timestamptz,
        title -> Text,
        module_id -> Text,
        credits -> Nullable<Int4>,
        content -> Text,
    }
}

diesel::joinable!(module_menu_module -> module_menu (module_menu_id));
diesel::joinable!(module_menu_module -> modules (module_id));

diesel::allow_tables_to_appear_in_same_query!(
    module_menu,
    module_menu_module,
    modules,
);
