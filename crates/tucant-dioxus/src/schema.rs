// @generated automatically by Diesel CLI.

diesel::table! {
    anmeldungen (url) {
        url -> Text,
        name -> Text,
        parent -> Nullable<Text>,
    }
}

diesel::table! {
    anmeldungen_entries (anmeldung, module_url) {
        anmeldung -> Nullable<Text>,
        module_url -> Text,
        id -> Text,
        name -> Text,
    }
}

diesel::joinable!(anmeldungen_entries -> anmeldungen (anmeldung));

diesel::allow_tables_to_appear_in_same_query!(
    anmeldungen,
    anmeldungen_entries,
);
