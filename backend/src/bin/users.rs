use self::models::*;
use diesel::prelude::*;
use backend::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for post in results {
        println!("{}", post.user_id);
        println!("{}", post.username);
        println!("^^^");
    }
}