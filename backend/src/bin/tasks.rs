use diesel::prelude::*;
use backend::*;
use backend::models::Task;

fn main() {
    use self::schema::tasks::dsl::*;

    let connection = &mut establish_connection();

    match tasks.limit(5).select(Task::as_select()).load::<Task>(connection) {
        Ok(results) => {
            println!("Displaying {} tasks", results.len());
            for task in results {
                println!("Task ID: {}", task.task_id);
                println!("User ID: {:?}", task.user_id);
                println!("Title: {}", task.title);
                println!("Description: {:?}", task.description);
                println!("Due Date: {:?}", task.due_date);
                println!("Completed: {:?}", task.completed);
                println!("Created At: {:?}", task.created_at);
                println!("Updated At: {:?}", task.updated_at);
                println!("^^^");
            }
        }
        Err(err) => eprintln!("Error loading tasks: {:?}", err),
    }
}
