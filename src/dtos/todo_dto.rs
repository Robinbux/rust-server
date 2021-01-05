use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTodoDTO {
    pub(crate) todo_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoDTO {
    pub(crate) completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoDTO {
    pub(crate) id: u32,
    pub(crate) todo_message: String,
    pub(crate) completed: bool,
}
