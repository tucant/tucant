// @generated automatically by Diesel CLI.

diesel::table! {
    anmeldungen_entries (semester, anmeldung, id) {
        semester -> Text,
        anmeldung -> Text,
        module_url -> Text,
        id -> Text,
        name -> Text,
        credits -> Integer,
        state -> Text,
    }
}

diesel::table! {
    anmeldungen_plan (url) {
        url -> Text,
        name -> Text,
        parent -> Nullable<Text>,
        min_cp -> Integer,
        max_cp -> Nullable<Integer>,
        min_modules -> Integer,
        max_modules -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(anmeldungen_entries, anmeldungen_plan,);
