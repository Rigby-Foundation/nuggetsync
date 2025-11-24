// @generated automatically by Diesel CLI.

diesel::table! {
    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        hash -> Text,
        name -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(profiles, users,);
