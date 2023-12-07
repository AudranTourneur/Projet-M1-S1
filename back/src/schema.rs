// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        salt -> Text,
        updated_at -> Timestamp,
    }
}
