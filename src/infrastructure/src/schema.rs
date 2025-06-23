// @generated automatically by Diesel CLI.

diesel::table! {
    tickets (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 255]
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
