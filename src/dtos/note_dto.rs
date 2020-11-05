use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateNoteDTO {
    pub(crate) note_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNoteDTO {
    pub(crate) note_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteDTO {
    pub(crate) id: u32,
    pub(crate) note_message: String,
}
