use crate::controller::controller::Controller;
use crate::dtos::note_dto;
use crate::server::request::Request;
use crate::server::request::Response;
use crate::services::notes_service::NotesService;
use crate::services::error_service::ErrorService;
use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;

use serde_json::Result;

pub struct NotesController {
    logger: Logger,
    error_service: ErrorService,
    notes_service: NotesService,
}

// PATH: /notes
impl NotesController {
    pub fn new() -> NotesController {
        let logger = Logger::new(String::from("NotesController"));
        let error_service = ErrorService::new();
        let notes_service = NotesService::new();
        NotesController {
            logger,
            notes_service,
        }
    }

    // POST
    // PATH: /
    pub fn create_note(&self, request: &mut Request) -> Response {
        let create_note_dto = match serde_json::from_str::<CreateNoteDTO>(request.payload) {
            Ok(dto) => dto,
            Err => {
                return self.error_service.serve_400_response()
            }
        }

        let result = self.notes_service.create_note(create_note_dto);
        if (result.is_err()) {
            return self.error_service.serve_500_response()
        }
        Response::new(
            result.unwrap().as_ref().to_vec(),
            ContentType::JSON,
            HTTPStatusCodes::Created,
        )
    }

    // PUT
    // PATH: /$NOTE_ID
    pub fn update_note(&self, request: &mut Request) -> Vec<u8> {
        let update_note_dto = match serde_json::from_str::<UpdateNoteDTO>(request.payload) {
            Ok(dto) => dto,
            Err => {
                return self.error_service.serve_400_response()
            }
        }

        let result = self.notes_service.update_note(update_note_dto, note_id);
        if (result.is_err()) {
            return self.error_service.serve_500_response()
        }
        Response::new(
            result.unwrap().as_ref().to_vec(),
            ContentType::JSON,
            HTTPStatusCodes::Created,
        )
    }

    // GET
    // PATH: /
    pub fn get_all_notes(&self, request: &mut Request) -> Vec<u8> {
        self.notes_service.get_all_notes()
    }

    // GET
    // PATH: /$NOTE_ID
    pub fn delete_note(&self, request: &mut Request) -> Vec<u8> {
        self.notes_service.delete_note()
    }

    fn get_note_id_from_request(request: &Request) -> u32 {
        
    }
}

impl Controller for NotesController {
    fn execute_request(&self, request: &mut Request) -> Result<Vec<u8>, Vec<u8>> {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        let route_beginning = BaseController::extract_parent_path(&request.current_child_path);
        return match route_beginning {
            "console" => Ok(self.console()),
            _ => Err(BaseController::serve_404_page()),
        };
    }

    fn get_content_type_for_path(&self, path: &str) -> ContentType {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "console" => ContentType::HTML,
            _ => ContentType::HTML,
        };
    }
}
