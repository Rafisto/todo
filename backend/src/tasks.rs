use std::io;
use diesel::prelude::*;
use backend::*;
use backend::models::{NewTask, Task};
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

pub fn forge_task(title_str: &str) -> NewTask {
    return NewTask {
        title: title_str,
        description: "",
        completed: false,
    };
}

pub fn add_element_to_list(task: NewTask) -> Result<bool, io::Error> {
    let connection = &mut establish_connection();
    let task = diesel::insert_into(tasks::table)
        .values(&task)
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