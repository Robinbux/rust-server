use crate::database::models::{Note, NewNote};
use crate::schema::notes::dsl::*;
use diesel::RunQueryDsl;
use diesel::associations::HasTable;
use crate::database::db_handler::db_handler::establish_connection;

pub fn create_note<'a>(note_message_str: &'a str) -> Note {
    use crate::schema::notes;

    let new_note = NewNote {
        note_message: note_message_str,
    };

    let connection = establish_connection();

    diesel::insert_into(notes::table)
        .values(&new_note)
        .get_result(&connection)
        .expect("Error saving new note")
}

pub fn get_notes() -> Vec<Note> {
    let connection = establish_connection();
    notes::table().load::<Note>(&connection).expect("Error retrieving notes")
}