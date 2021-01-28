use crate::schema::todos;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, PartialEq, Debug)]
pub struct Todo {
    pub id: i32,
    pub todo_message: String,
    pub completed: bool,
}

#[derive(Insertable, Queryable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub todo_message: &'a str,
    pub completed: bool,
}
