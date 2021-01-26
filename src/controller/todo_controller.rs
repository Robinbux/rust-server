use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::dtos::todo_dto::{CreateTodoDTO, UpdateTodoDTO};
use crate::enums::content_type::ContentType;
use crate::enums::http_methods::HttpMethod;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::services::todo_service::TodoService;
use crate::utils::logger::Logger;
use std::str;

#[derive(Clone)]
pub struct TodoController {
    #[allow(dead_code)]
    logger: Logger,
    error_service: ErrorService,
    todo_service: TodoService,
}

// PATH: /todo
impl TodoController {
    pub fn new() -> TodoController {
        let logger = Logger::new(String::from("TodoController"));
        let error_service = ErrorService::new();
        let todo_service = TodoService::new();
        TodoController {
            logger,
            error_service,
            todo_service,
        }
    }


    // POST
    // PATH: /
    pub fn create_todo(&self, request: Request) -> Response {
        if request.payload.is_none() {
            return self
                .error_service
                .serve_400_response("Incorrect Payload Structure!".to_string());
        }
        let payload = request.payload.unwrap();
        let json_result = match serde_json::from_str::<CreateTodoDTO>(&payload) {
            Ok(dto) => dto,
            Err(_) => {
                return self
                    .error_service
                    .serve_400_response("Incorrect Payload Structure!".to_string());
            }
        };

        let result = self.todo_service.create_todo(json_result);
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to create Todo!".to_string());
        }
        let result_str = serde_json::to_string(&result.unwrap()).unwrap();
        let result_ref: &[u8] = result_str.as_ref();
        Response::new(
            result_ref.to_vec(),
            ContentType::JSON,
            HTTPStatusCodes::Created,
        )
    }

    // PUT
    // PATH: /$TODO_ID
    pub fn update_todo(&self, request: Request, todo_id: i32) -> Response {
        if request.payload.is_none() {
            return self
                .error_service
                .serve_400_response("Incorrect Payload Structure!".to_string());
        }
        let payload = request.payload.unwrap();

        let update_todo_dto =
            match serde_json::from_str::<UpdateTodoDTO>(&payload) {
                Ok(dto) => dto,
                Err(_) => {
                    return self
                        .error_service
                        .serve_400_response("Incorrect Payload Structure!".to_string());
                }
            };

        let result = self.todo_service.update_todo(update_todo_dto, todo_id);
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to update Todo!".to_string());
        }
        let result_str = serde_json::to_string(&result.unwrap()).unwrap(); // TODO: Change!
        let result_ref: &[u8] = result_str.as_ref();
        Response::new(
            result_ref.to_vec(),
            ContentType::JSON,
            HTTPStatusCodes::Created,
        )
    }

    // GET
    // PATH: /

    pub fn get_all_todos(&self) -> Response {
        let result = self.todo_service.get_all_todos();
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to retrieve todos!".to_string());
        }
        let result_str = serde_json::to_string(&result.unwrap()).unwrap();
        let result_ref: &[u8] = result_str.as_ref();
        Response::new(result_ref.to_vec(), ContentType::JSON, HTTPStatusCodes::Ok)
    }

    // DELETE
    // PATH: /$TODO_ID
    pub fn delete_todos(&self, todo_id: i32) -> Response {
        let result = self.todo_service.delete_todo(todo_id);
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to delete Todo!".to_string());
        }
        let json_response = String::from("{\"temp\":\"Change\"}");
        let result_ref: &[u8] = json_response.as_ref();
        Response::new(
            result_ref.to_vec(), // TODO: See how to handle no response
            ContentType::JSON,
            HTTPStatusCodes::Ok,
        )
    }

    fn get_id_from_request(request: &Request) -> Result<i32, String> {
        let id_option = request.current_child_path.split('/').last();
        if id_option.is_none() {
            return Err("".to_string());
        }
        let id_result = id_option.unwrap().parse::<i32>();
        if id_result.is_err() {
            return Err("".to_string());
        }
        Ok(id_result.unwrap())
    }

    fn error_or_todo_id_request(&self, request: Request) -> Response {
        let todo_id_result = TodoController::get_id_from_request(&request);
        match todo_id_result {
            Ok(id) => {
                if request.http_method == HttpMethod::PUT {
                    self.update_todo(request, id)
                } else {
                    self.delete_todos(id)
                }
            }
            Err(_) => self.error_service.serve_400_response(String::from("can't handle request")),
        }
    }
}

impl Controller for TodoController {
    fn execute_request(&self, mut request: Request) -> Response {
        match request.http_method
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        let route_beginning = BaseController::extract_parent_path(&request.current_child_path);
        match route_beginning {
            "" => {
                if request.http_method == HttpMethod::GET {
                    self.get_all_todos()
                } else {
                    self.create_todo(request)
                }
            }
            _ => self.error_or_todo_id_request(request),
        }
    }
}

mod tests {

    use super::*;
    /*
    #[cfg(test)]
    #[test]
    #[allow(dead_code)]

    fn extract_child_long_path() {
        let request_string: &str = "GET /favicon.ico HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"todo_message\":\"pee\"}";

        let todo_controller = TodoController::new();
        let request = Request::new(String::from(request_string));
        let payload = TodoController::get_trimmed_payload_from_request(&todo_controller, &request).unwrap();
        let expected_payload = r#"{"todo_message":"pee"}"#;
        assert_eq!(expected_payload, request.payload.unwrap())
    }*/
}
