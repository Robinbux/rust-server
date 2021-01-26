mod services;
pub mod net;
mod utils;
mod path_handler;
mod enums;
mod dtos;
pub mod database;
mod controller;
mod schema;
mod thread_pool;
#[macro_use]
extern crate diesel;


mod TodoAPI {
    pub use crate::controller::todo_controller::TodoController;
    use crate::net::request::Request;
    use crate::net::response::Response;

    pub fn create_todo(todoController: TodoController, request: Request) -> Response {
        todoController.create_todo(request)
    }

    /*pub fn update_todo(todoController: TodoController, update_todo_dto: UpdateTodoDTO, todo_id: i32) -> Result<(), diesel::result::Error> {
        update_todo(update_todo_dto.completed, todo_id)
    }

    pub fn get_all_todos(todoController: TodoController) -> Result<Vec<Todo>, diesel::result::Error> {
        get_todos()
    }

    pub fn delete_todo(todoController: TodoController, todo_id: i32) -> Result<(), diesel::result::Error> {
        delete_todo(todo_id)
    }*/
}