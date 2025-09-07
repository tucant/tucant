// @generated automatically by Diesel CLI.

diesel::table! {
    anmeldungen (semester, url) {
        semester -> Text,
        url -> Text,
        name -> Text,
        parent -> Nullable<Text>,
    }
}

diesel::table! {
    anmeldungen_entries (anmeldung, module_url) {
        semester -> Text,
        anmeldung -> Text,
        module_url -> Text,
        id -> Text,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    anmeldungen,
    anmeldungen_entries,
);
