pub mod tasks;
use tasks::{get_todo_list, add_element_to_list, remove_element_from_list, update_task};
use rocket::serde::json::{json, Json, Value};
use backend::models::{NewTask, UpdateTask};

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "API is working..."
}

#[get("/todo")]
fn get_todo() -> Value {
    return match get_todo_list() {
        Ok(result) => json!(result),
        _ => json!({"message": "Unable to fetch the todo list."})
    }
}

#[put("/todo", data = "<element>")]
fn add_todo(element: Json<NewTask<'_>>) -> Value {
   return match add_element_to_list(element) {
       Ok(_result) => json!({"message": "Added correctly."}),
       _ => json!({"message": "Error adding element."})
   }
}

#[delete("/todo/<task_id>")]
fn delete_todo(task_id: i32) -> Value {
    return match remove_element_from_list(task_id) {
        Ok(_result) => json!({"message": "Removed correctly."}),
        _ => json!({"message": "Error removing element."})
    }
}

#[patch("/todo/<task_id>", data = "<updated_task>")]
fn update_todo(task_id: i32, updated_task: Json<UpdateTask>) -> Value {
    match update_task(task_id, updated_task) {
        Ok(_) => json!({"message": "Updated correctly."}),
        _ => json!({"message": "Error updating element."}),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, get_todo, add_todo, delete_todo, update_todo])
}