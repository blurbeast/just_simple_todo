// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        public_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        alias -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
