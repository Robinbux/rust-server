use postgres::{Client, Error, NoTls};
use crate::utils::logger::Logger;
use crate::dtos::note_dto::{NoteDTO, CreateNoteDTO, UpdateNoteDTO};
use crate::database::models::Note;
use crate::database::repositories::note_repository::{create_note, get_notes};

pub struct NotesService {
    #[allow(dead_code)]
    logger: Logger,
}

impl NotesService {
    pub fn new() -> NotesService {
        let logger = Logger::new(String::from("NotesService"));

        NotesService { logger }
    }

    pub fn create_note(&mut self, create_note_dto: CreateNoteDTO) -> Result<Note, Error> {
        Ok(create_note(&create_note_dto.note_message))
    }

    pub fn update_note(
        &mut self,
        update_note_dto: UpdateNoteDTO,
        note_id: u32,
    ) {

    }

    pub fn get_all_notes(&mut self) -> Result<Vec<Note>, Error> {
        let notes = get_notes();
        Ok(notes)
    }

    pub fn delete_note(&mut self, note_id: u32) -> Result<(), Error> {
        Ok(())
    }
}
