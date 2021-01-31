use crate::database::models::Todo;
use crate::database::repositories::todo_repository::{
    create_todo, delete_todo, get_todos, update_todo,
};
use crate::dtos::todo_dto::{CreateTodoDTO, UpdateTodoDTO};
use crate::utils::logger::Logger;
use crate::database::db_handler::db_handler::establish_connection;
use diesel::PgConnection;

#[derive(Clone)]
pub struct TodoService {
    #[allow(dead_code)]
    logger: Logger,
}

impl TodoService {
    pub fn new() -> TodoService {
        let logger = Logger::new(String::from("TodoService"));
        TodoService { logger }
    }

    pub fn create_todo(&self, create_todo_dto: CreateTodoDTO) -> Result<Todo, diesel::result::Error> {
        let db_connection = establish_connection();
        create_todo(&db_connection, &create_todo_dto.todo_message)
    }

    pub fn update_todo(&self, update_todo_dto: UpdateTodoDTO, todo_id: i32) -> Result<(), diesel::result::Error> {
        let db_connection = establish_connection();
        update_todo(&db_connection, update_todo_dto.completed, todo_id)
    }

    pub fn get_all_todos(&self) -> Result<Vec<Todo>, diesel::result::Error> {
        let db_connection = establish_connection();
        get_todos(&db_connection)
    }

    pub fn delete_todo(&self, todo_id: i32) -> Result<usize, diesel::result::Error> {
        let db_connection = establish_connection();
        delete_todo(&db_connection, todo_id)
    }
}
