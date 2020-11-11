use serde::{Deserialize, Serialize};
use crate::schema::notes;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub note_message: String
}

#[derive(Insertable, Queryable)]
#[table_name="notes"]
pub struct NewNote<'a> {
    pub note_message: &'a str,
}