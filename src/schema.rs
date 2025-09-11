// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        alias -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}
