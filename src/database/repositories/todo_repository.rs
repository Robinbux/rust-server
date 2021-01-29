use crate::database::db_handler::db_handler::establish_connection;
use crate::database::models::{NewTodo, Todo};
use crate::schema::todos::dsl::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::RunQueryDsl;

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

mod tests {
    use super::*;
    use std::process::Command;
    use std::fs;
    use crate::schema::todos::dsl::*;

    fn setup_db() -> PgConnection {
        //overwrite .env file to use test db
        fs::write(".env", "DATABASE_URL=postgres://postgres:password@localhost/todos_test");
        PgConnection::establish("postgres://postgres:password@localhost/todos_test").expect("Error connecting to DB")
    }

    fn clean_up() {
        Command::new("diesel").args(&["migration", "redo"]).spawn().expect("migration failed");
        fs::write(".env", "DATABASE_URL=postgres://postgres:password@localhost/postgres");
    }

    #[test]
    fn test_db(){
        let connection = setup_db();

        // test create_todo
        let todo_str = "test";
        create_todo(todo_str);
        let expected_todo = Todo{id:1, todo_message: "test".to_string(), completed: false };
        let actual_todos = todos::table().load::<Todo>(&connection).unwrap();
        assert_eq!(vec![expected_todo], actual_todos);

        // test update_todo
        update_todo(true, 1);
        let expected_todo = Todo{id:1, todo_message: "test".to_string(), completed: true };
        let actual_todos = todos::table().load::<Todo>(&connection).unwrap();
        assert_eq!(vec![expected_todo], actual_todos);

        // test delete & get todo
        delete_todo(1);
        let result = get_todos().unwrap();
        assert_eq!(result, vec![]);

        clean_up();
    }
}
