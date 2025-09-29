// @generated automatically by Diesel CLI.

diesel::table! {
    anmeldungen_entries (course_of_study, available_semester, anmeldung, id) {
        course_of_study -> Text,
        available_semester -> Text,
        anmeldung -> Text,
        module_url -> Text,
        id -> Text,
        name -> Text,
        credits -> Integer,
        state -> Text,
        year -> Nullable<Integer>,
        semester -> Nullable<Text>,
    }
}

diesel::table! {
    anmeldungen_plan (course_of_study, url) {
        course_of_study -> Text,
        url -> Text,
        name -> Text,
        parent -> Nullable<Text>,
        min_cp -> Integer,
        max_cp -> Nullable<Integer>,
        min_modules -> Integer,
        max_modules -> Nullable<Integer>,
    }
}

diesel::table! {
    store (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    anmeldungen_entries,anmeldungen_plan,store,);
