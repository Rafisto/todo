use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub task_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub completed: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Insertable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub due_date: Option<NaiveDate>,
    pub completed: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateTask<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub due_date: Option<NaiveDate>,
    pub completed: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}