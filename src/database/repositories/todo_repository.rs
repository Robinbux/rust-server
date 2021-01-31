use crate::database::db_handler::db_handler::establish_connection;
use crate::database::models::{NewTodo, Todo};
use crate::schema::todos::dsl::*;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn create_todo<'a>(connection: &PgConnection, todo_message_str: &'a str) -> Result<Todo, diesel::result::Error> {
    let new_todo = NewTodo {
        todo_message: todo_message_str,
        completed: false,
    };

    let new_todo = diesel::insert_into(todos)
        .values(&new_todo)
        .get_result(connection)?;
    Ok(new_todo)
}

pub fn update_todo(connection: &PgConnection, update_completed: bool, todo_id: i32) -> Result<(), diesel::result::Error> {
    let target_todo = todos.filter(id.eq(todo_id));

    diesel::update(target_todo)
        .set(completed.eq(update_completed))
        .execute(connection)?;
    Ok(())
}

pub fn get_todos(connection: &PgConnection) -> Result<Vec<Todo>, diesel::result::Error> {
    let all_todos = todos::table()
        .load::<Todo>(connection)?;
    Ok(all_todos)
}

pub fn delete_todo(connection: &PgConnection, todo_id: i32) -> Result<usize, diesel::result::Error> {
    diesel::delete(todos)
        .filter(id.eq(todo_id))
        .execute(connection)
}

mod tests {
    use super::*;
    use std::process::Command;
    use std::fs;

    #[test]
    fn create() {
        let db_connection = PgConnection::establish("postgres://postgres:password@localhost/todos_test").expect("Error connecting to DB");

        let new_todo = create_todo(&db_connection, "test create").expect("error loading todo");
        let expected_todo = Todo{id:new_todo.id, todo_message: "test create".to_string(), completed: false };
        let actual_todo = todos.filter(id.eq(new_todo.id)).load::<Todo>(&db_connection).expect("error loading todo");
        assert_eq!(vec![expected_todo], actual_todo);

        diesel::delete(todos)
            .filter(id.eq(new_todo.id))
            .execute(&db_connection).expect("error deleting todo");
    }

    #[test]
    fn update_completed() {
        let db_connection = PgConnection::establish("postgres://postgres:password@localhost/todos_test").expect("Error connecting to DB");
        let new_todo = create_todo(&db_connection, "test update 1").expect("error loading todo");

        update_todo(&db_connection, true, new_todo.id);
        let expected_todo = Todo{id:new_todo.id, todo_message: "test update 1".to_string(), completed: true };
        let actual_todo = todos.filter(id.eq(new_todo.id)).load::<Todo>(&db_connection).expect("error loading todo");
        assert_eq!(vec![expected_todo], actual_todo);

        diesel::delete(todos)
            .filter(id.eq(new_todo.id))
            .execute(&db_connection).expect("error deleting todo");
    }

    #[test]
    fn update_uncompleted() {
        let db_connection = PgConnection::establish("postgres://postgres:password@localhost/todos_test").expect("Error connecting to DB");
        let new_todo = create_todo(&db_connection, "test update 2").expect("error loading todo");

        update_todo(&db_connection, false, new_todo.id);
        let expected_todo = Todo{id:new_todo.id, todo_message: "test update 2".to_string(), completed: false };
        let actual_todo = todos.filter(id.eq(new_todo.id)).load::<Todo>(&db_connection).expect("error loading todo");
        assert_eq!(vec![expected_todo], actual_todo);

        diesel::delete(todos)
            .filter(id.eq(new_todo.id))
            .execute(&db_connection).expect("error deleting todo");
    }

    #[test]
    fn get() {
        let db_connection = PgConnection::establish("postgres://postgres:password@localhost/todos_test").expect("Error connecting to DB");

        get_todos(&db_connection).unwrap();
    }

    #[test]
    fn delete() {
        let db_connection = PgConnection::establish("postgres://postgres:password@localhost/todos_test").expect("Error connecting to DB");
        let new_todo = create_todo(&db_connection, "test delete").expect("error loading todo");

        delete_todo(&db_connection, new_todo.id);
        let result = todos.filter(id.eq(new_todo.id)).load::<Todo>(&db_connection).expect("error loading todo");
        assert!(result.len() == 0);
    }

    /*#[should_panic]
    #[test]
    fn delete_non_existing() {
        let db_connection = PgConnection::establish("postgres://postgres:password@localhost/todos_test").expect("Error connecting to DB");

        delete_todo(&db_connection, 0).unwrap();
    }*/


/*#[cfg]
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
}*/
}
