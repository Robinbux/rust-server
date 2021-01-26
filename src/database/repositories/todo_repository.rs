use crate::database::db_handler::db_handler::establish_connection;
use crate::database::models::{NewTodo, Todo};
use crate::schema::todos::dsl::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use postgres::Error;

pub fn create_todo<'a>(todo_message_str: &'a str) -> Result<Todo, diesel::result::Error> {
    use crate::schema::todos;

    let new_todo = NewTodo {
        todo_message: todo_message_str,
        completed: false,
    };
    let connection = establish_connection();

    let new_todo = diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(&connection)?;
    Ok(new_todo)
}

pub fn update_todo(update_completed: bool, todo_id: i32) -> Result<(), diesel::result::Error> {
    let connection = establish_connection();
    let target_todo = todos.filter(id.eq(todo_id));

    diesel::update(target_todo)
        .set(completed.eq(update_completed))
        .execute(&connection)?;
    Ok(())
}

pub fn get_todos() -> Result<Vec<Todo>, diesel::result::Error> {
    let connection = establish_connection();
    let all_todos = todos::table()
        .load::<Todo>(&connection)?;
    Ok(all_todos)
}

pub fn delete_todo(todo_id: i32) -> Result<(), diesel::result::Error> {
    use crate::schema::todos;

    let connection = establish_connection();
    diesel::delete(todos::table)
        .filter(id.eq(todo_id))
        .execute(&connection)?;
    Ok(())
}
