use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub task_id: i32,
    pub user_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}