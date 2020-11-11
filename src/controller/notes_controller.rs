use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::dtos::note_dto::CreateNoteDTO;
use crate::dtos::note_dto::UpdateNoteDTO;
use crate::enums::content_type::ContentType;
use crate::enums::http_methods::HttpMethod;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::services::notes_service::NotesService;
use crate::utils::logger::Logger;

pub struct NotesController {
    #[allow(dead_code)]
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
            error_service,
            notes_service,
        }
    }

    // POST
    // PATH: /
    pub fn create_note(&mut self, request: &Request) -> Response {
        let create_note_dto = match serde_json::from_str::<CreateNoteDTO>(&request.payload) {
            Ok(dto) => dto,
            Err(_) => {
                return self
                    .error_service
                    .serve_400_response("Incorrect Payload Structure!".to_string());
            }
        };

        let result = self.notes_service.create_note(create_note_dto);
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to create note!".to_string());
        }
        let result_str = serde_json::to_string(&result.unwrap()).unwrap(); // TODO: Change!
        let result_ref: &[u8] = result_str.as_ref();
        Response::new(
            result_ref.to_vec(),
            ContentType::JSON,
            HTTPStatusCodes::Created,
        )
    }

    // PUT
    // PATH: /$NOTE_ID
    pub fn update_note(&mut self, request: &Request) -> Response {
        let update_note_dto = match serde_json::from_str::<UpdateNoteDTO>(&request.payload) {
            Ok(dto) => dto,
            Err(_) => {
                return self
                    .error_service
                    .serve_400_response("Incorrect Payload Structure!".to_string());
            }
        };

        let note_id = NotesController::get_note_id_from_request(&request);
        if note_id.is_err() {
            return self
                .error_service
                .serve_400_response("Unable to parse provided id!".to_string());
        }

        let result = self
            .notes_service
            .update_note(update_note_dto, note_id.unwrap());
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to update note!".to_string());
        }
        let result_str = serde_json::to_string(&result.unwrap()).unwrap();
        let result_ref: &[u8] = result_str.as_ref();
        Response::new(result_ref.to_vec(), ContentType::JSON, HTTPStatusCodes::Ok)
    }

    // GET
    // PATH: /
    pub fn get_all_notes(&mut self) -> Response {
        let result = self.notes_service.get_all_notes();
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to retrieve notes!".to_string());
        }
        let result_str = serde_json::to_string(&result.unwrap()).unwrap();
        let result_ref: &[u8] = result_str.as_ref();
        Response::new(result_ref.to_vec(), ContentType::JSON, HTTPStatusCodes::Ok)
    }

    // DELETE
    // PATH: /$NOTE_ID
    pub fn delete_note(&mut self, request: &Request) -> Response {
        let note_id = NotesController::get_note_id_from_request(request);
        if note_id.is_err() {
            return self
                .error_service
                .serve_400_response("Unable to parse provided id!".to_string());
        }

        let result = self.notes_service.delete_note(note_id.unwrap());
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to delete note!".to_string());
        }
        let json_response = String::from("{\"temp\":\"Change\"}");
        let result_ref: &[u8] = json_response.as_ref();
        Response::new(
            result_ref.to_vec(), // TODO: See how to handle no response
            ContentType::JSON,
            HTTPStatusCodes::Ok,
        )
    }

    fn get_note_id_from_request(request: &Request) -> Result<u32, String> {
        let id_option = request.current_child_path.split("/").last();
        if id_option.is_none() {
            return Err("".to_string());
        }
        let id_result = id_option.unwrap().parse::<u32>();
        if id_result.is_err() {
            return Err("".to_string());
        }
        Ok(id_result.unwrap())
    }

    fn note_id_request_or_error(&mut self, request: &Request) -> Response {
        let note_id_result = NotesController::get_note_id_from_request(&request);
        return match note_id_result {
            Ok(_id) => {
                if request.http_method == HttpMethod::UPDATE {
                    self.update_note(&request)
                } else {
                    self.delete_note(&request)
                }
            }
            Err(_) => self.error_service.serve_404_page(),
        };
    }
}

impl Controller for NotesController {
    fn execute_request(&mut self, mut request: &mut Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        let route_beginning = BaseController::extract_parent_path(&request.current_child_path);
        return match route_beginning {
            "" => {
                if request.http_method == HttpMethod::GET {
                    self.get_all_notes()
                } else {
                    self.create_note(&mut request)
                }
            }
            _ => self.note_id_request_or_error(&mut request),
        };
    }
}
