use crate::database::models::Todo;
use crate::database::repositories::todo_repository::{
    create_todo, delete_todo, get_todos, update_todo,
};
use crate::dtos::todo_dto::{CreateTodoDTO, UpdateTodoDTO};
use crate::utils::logger::Logger;

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
        create_todo(&create_todo_dto.todo_message)
    }

    pub fn update_todo(&self, update_todo_dto: UpdateTodoDTO, todo_id: i32) -> Result<(), diesel::result::Error> {
        update_todo(update_todo_dto.completed, todo_id)
    }

    pub fn get_all_todos(&self) -> Result<Vec<Todo>, diesel::result::Error> {
        get_todos()
    }

    pub fn delete_todo(&self, todo_id: i32) -> Result<(), diesel::result::Error> {
        delete_todo(todo_id)
    }
}
