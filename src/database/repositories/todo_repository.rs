use crate::database::db_handler::db_handler::establish_connection;
use crate::database::models::{NewTodo, Todo};
use crate::schema::todos::dsl::*;
use diesel::associations::HasTable;
use diesel::RunQueryDsl;
use postgres::Error;
use diesel::prelude::*;

pub fn create_todo<'a>(todo_message_str: &'a str) -> Todo {
    use crate::schema::todos;

    let new_todo = NewTodo {
        todo_message: todo_message_str,
        completed: false,
    };

    let connection = establish_connection();

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(&connection)
        .expect("Error saving new Todo")
}

pub fn update_todo(update_completed: bool, todo_id: i32) -> Result<(), Error> {
    let connection = establish_connection();
    let target_todo = todos.filter(id.eq(todo_id));

    diesel::update(target_todo)
        .set(completed.eq(update_completed))
        .execute(&connection)
        .expect("Unable to update Todo");

    Ok(())
}

pub fn get_todos() -> Vec<Todo> {
    let connection = establish_connection();
    todos::table()
        .load::<Todo>(&connection)
        .expect("Error retrieving Todos")
}

pub fn delete_todo(todo_id: i32) -> Result<(), Error> {
    use crate::schema::todos;

    let connection = establish_connection();
    diesel::delete(todos::table)
        .filter(id.eq(todo_id))
        .execute(&connection)
        .expect("Error deleting Todo");

    Ok(())
}
