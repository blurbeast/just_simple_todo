// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        public_id -> Uuid,
        created_at -> Nullable<Varchar>,
        completed -> Bool,
        user_id -> Int4,
        updated_at -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        alias -> Varchar,
        created_at -> Nullable<Varchar>,
        password -> Varchar,
    }
}

diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(todos, users,);
