// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (task_id) {
        task_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        completed -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
