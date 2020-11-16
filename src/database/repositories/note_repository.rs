use crate::database::db_handler::db_handler::establish_connection;
use crate::database::models::{NewNote, Note};
use crate::schema::notes::dsl::*;
use diesel::associations::HasTable;
use diesel::RunQueryDsl;
use postgres::Error;

use diesel::delete;
use diesel::prelude::*;

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

pub fn update_note(note_message_str: &str, note_id: i32) -> Result<(), Error> {
    use crate::schema::notes;

    let connection = establish_connection();

    let target_note = notes.filter(id.eq(note_id));

    diesel::update(target_note)
        .set(note_message.eq(note_message_str))
        .execute(&connection)
        .expect("Unable to update note");

    Ok(())
}

pub fn get_notes() -> Vec<Note> {
    let connection = establish_connection();
    notes::table()
        .load::<Note>(&connection)
        .expect("Error retrieving notes")
}

pub fn delete_note(note_id: i32) -> Result<(), Error> {
    use crate::schema::notes;

    let connection = establish_connection();
    diesel::delete(notes::table)
        .filter(id.eq(note_id))
        .execute(&connection)
        .expect("Error deleting note");

    Ok(())
}
