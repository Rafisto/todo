// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (task_id) {
        task_id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        completed -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 100]
        password_hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
