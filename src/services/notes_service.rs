use crate::database::models::Note;
use crate::database::repositories::note_repository::{
    create_note, delete_note, get_notes, update_note,
};
use crate::dtos::note_dto::{CreateNoteDTO, UpdateNoteDTO};
use crate::utils::logger::Logger;
use postgres::Error;

#[derive(Clone)]
pub struct NotesService {
    #[allow(dead_code)]
    logger: Logger,
}

impl NotesService {
    pub fn new() -> NotesService {
        let logger = Logger::new(String::from("NotesService"));

        NotesService { logger }
    }

    pub fn create_note(&self, create_note_dto: CreateNoteDTO) -> Result<Note, Error> {
        Ok(create_note(&create_note_dto.note_message))
    }

    pub fn update_note(&self, update_note_dto: UpdateNoteDTO, note_id: i32) -> Result<(), Error> {
        update_note(&update_note_dto.note_message, note_id)
    }

    pub fn get_all_notes(&self) -> Result<Vec<Note>, Error> {
        let notes = get_notes();
        Ok(notes)
    }

    pub fn delete_note(&self, note_id: i32) -> Result<(), Error> {
        delete_note(note_id)
    }
}
