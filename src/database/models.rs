use crate::schema::notes;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub note_message: String,
}

#[derive(Insertable, Queryable)]
#[table_name = "notes"]
pub struct NewNote<'a> {
    pub note_message: &'a str,
}
