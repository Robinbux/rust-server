use postgres::{Client, Error, NoTls};
use crate::utils::logger::Logger;
use crate::dtos::note_dto::{NoteDTO, CreateNoteDTO, UpdateNoteDTO};

pub struct NotesService {
    #[allow(dead_code)]
    logger: Logger,
    client: Client,
}

impl NotesService {
    pub fn new() -> NotesService {
        let logger = Logger::new(String::from("NotesService"));
        let client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

        NotesService { logger, client }
    }

    pub fn create_note(&mut self, create_note_dto: CreateNoteDTO) -> Result<NoteDTO, Error> {
        let row = self.client.query_one(
            "INSERT INTO notes (note_message) VALUES ($1)",
            &[&create_note_dto.note_message],
        )?;
        Ok(NoteDTO {
            id: row.get(0),
            note_message: row.get(1),
        })
    }

    pub fn update_note(
        &mut self,
        update_note_dto: UpdateNoteDTO,
        note_id: u32,
    ) -> Result<NoteDTO, Error> {
        let row = self.client.query_one(
            "UPDATE notes n
                SET n.note_message = $1
             WHERE n.id = $2",
            &[&update_note_dto.note_message, &note_id],
        )?;
        Ok(NoteDTO {
            id: row.get(0),
            note_message: row.get(1),
        })
    }

    pub fn get_all_notes(&mut self) -> Result<Vec<NoteDTO>, Error> {
        let mut notes: Vec<NoteDTO> = Vec::new();
        for row in self
            .client
            .query("SELECT id, note_message FROM notes", &[])
            .unwrap()
        {
            notes.push(NoteDTO {
                id: row.get(0),
                note_message: row.get(1),
            })
        }
        Ok(notes)
    }

    pub fn delete_note(&mut self, note_id: u32) -> Result<(), Error> {
        self.client
            .execute("DELETE * FROM notes n WHERE n.id = $1", &[&note_id])?;
        Ok(())
    }
}
