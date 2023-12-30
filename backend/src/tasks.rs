use std::io;
use diesel::prelude::*;
use rocket::serde::json::Json;
use backend::*;
use backend::models::{NewTask, Task, UpdateTask};
use backend::schema::tasks;
use backend::schema::tasks::dsl::*;

pub fn get_todo_list() -> Result<Vec<Task>, io::Error> {
    let connection = &mut establish_connection();
    match tasks.load::<Task>(connection) {
        Ok(tasks_vec) => Ok(tasks_vec),
        Err(e) => {
            eprintln!("Failed to fetch todo list: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, "Database error"))
        }
    }
}

pub fn add_element_to_list(task: Json<NewTask>) -> Result<bool, io::Error> {
    let connection = &mut establish_connection();
    let task = diesel::insert_into(tasks::table)
        .values(&*task)
        .returning(Task::as_returning())
        .get_result(connection);
    match task {
        Ok(_task) => Ok(true),
        Err(e) => {
            eprintln!("Failed to fetch todo list: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, "Database error"))
        }
    }
}

pub fn remove_element_from_list(index: i32) -> Result<(), io::Error> {
    let connection = &mut establish_connection();
    let deleted_rows = diesel::delete(tasks.filter(task_id.eq(index)))
        .execute(connection)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, format!("Task with id {} not found", index)))?;

    if deleted_rows == 0 {
        Err(io::Error::new(io::ErrorKind::Other, format!("Task with id {} not found", index)))
    } else {
        Ok(())
    }
}

pub fn update_task(index: i32, updated_task: Json<UpdateTask>) -> Result<(), io::Error> {
    let connection = &mut establish_connection();
    let updated_rows = diesel::update(tasks::table.filter(tasks::task_id.eq(index)))
        .set(updated_task.into_inner())
        .execute(connection)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, format!("Failed to update task with id {}", index)))?;

    if updated_rows == 0 {
        Err(io::Error::new(io::ErrorKind::Other, format!("Task with id {} not found", index)))
    } else {
        Ok(())
    }
}