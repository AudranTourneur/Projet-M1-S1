// @generated automatically by Diesel CLI.

diesel::table! {
    container_statistics (id, timestamp) {
        id -> Text,
        timestamp -> Timestamp,
        memory_usage -> Int4,
        cpu_usage -> Float8,
        io_usage_read -> Float8,
        io_usage_write -> Float8,
        network_usage_up -> Float8,
        network_usage_down -> Float8,
    }
}

diesel::table! {
    network_statistics (id, timestamp) {
        id -> Text,
        timestamp -> Timestamp,
        network_usage_up -> Float8,
        network_usage_down -> Float8,
    }
}

diesel::table! {
    sessions (session_token) {
        session_token -> Text,
        user_id -> Int4,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        salt -> Text,
        topology -> Jsonb,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    volume_statistics (id, timestamp) {
        id -> Text,
        timestamp -> Timestamp,
        disk_usage -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    container_statistics,
    network_statistics,
    sessions,
    users,
    volume_statistics,
);
