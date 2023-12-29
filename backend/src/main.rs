pub mod tasks;
use tasks::{get_todo_list, add_element_to_list, forge_task};
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Serialize,Deserialize};

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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    title: &'r str,
}

#[put("/todo", data = "<element>")]
fn add_todo(element: Json<Message<'_>>) -> Value {
   return match add_element_to_list(forge_task(element.title)) {
       Ok(_result) => json!({"message": "Added correctly."}),
       _ => json!({"message": "Error adding element."})
   }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, get_todo, add_todo])
}