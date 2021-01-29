use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::dtos::todo_dto::{CreateTodoDTO, UpdateTodoDTO};
use crate::enums::content_type::ContentType;
use crate::enums::http_methods::HttpMethod;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::services::error_service::ErrorService;
use crate::services::todo_service::TodoService;
use crate::utils::logger::Logger;
use std::num::ParseIntError;
use serde::Serialize;
use hyper::body::Buf;


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
        let json_result = match serde_json::from_str::<CreateTodoDTO>(&request.payload.unwrap()) {
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
        let new_todo_str = serde_json::to_string(&result.unwrap()).unwrap();
        println!("response payload: {}", new_todo_str);
        Response::new(
            new_todo_str.as_bytes().to_owned(),
            ContentType::JSON,
            HTTPStatusCodes::Created,
        )
    }

    // PUT
    // PATH: /$TODO_ID
    pub fn update_todo(&self, request: Request) -> Response {
        let todo_id = TodoController::get_id_from_request(&request);
        if todo_id.is_err() {
            return self.error_service.serve_400_response(todo_id.unwrap_err())
        };
        if request.payload.is_none() {
            return self
                .error_service
                .serve_400_response("Incorrect Payload Structure!".to_string())
        };
        let payload = request.payload.unwrap();
        println!("update response payload: {}", payload);

        let update_todo_dto =
            match serde_json::from_str::<UpdateTodoDTO>(&payload) {
                Ok(dto) => dto,
                Err(_) => {
                    return self
                        .error_service
                        .serve_400_response("Incorrect Payload Structure!".to_string());
                }
            };

        let result = self.todo_service.update_todo(update_todo_dto, todo_id.unwrap());
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to update Todo!".to_string());
        };
        Response::new(
            Vec::new(),
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
        println!("{}", result_str);
        let result_ref: &[u8] = result_str.as_ref();
        Response::new(result_ref.to_vec(), ContentType::JSON, HTTPStatusCodes::Ok)
    }

    // DELETE
    // PATH: /$TODO_ID
    pub fn delete_todo(&self, request: Request) -> Response {
        let todo_id = TodoController::get_id_from_request(&request);
        if todo_id.is_err() {
            return self.error_service.serve_400_response(todo_id.unwrap_err())
        };
        let result = self.todo_service.delete_todo(todo_id.unwrap());
        if result.is_err() {
            return self
                .error_service
                .serve_500_response("Unable to delete Todo!".to_string())
        };
        Response::new(
            Vec::new(), // TODO: See how to handle no response
            ContentType::JSON,
            HTTPStatusCodes::Ok,
        )
    }

    fn get_id_from_request(request: &Request) -> Result<i32, String> {
        let id_option = request.current_child_path.split('/').last();
        if id_option.is_none() {
            return Err(String::from("Todo id is missing!"))
        };
        let id = id_option.unwrap().parse::<i32>();
        if id.is_err(){
            return Err(String::from("Wrong id type"))
        };
        id.map_err(TodoController::stringify)
    }

    fn stringify(x: ParseIntError) -> String {
        format!("{:?}", x)
    }
}

impl Controller for TodoController {
    fn execute_request(&self, mut request: Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        match request.http_method {
            HttpMethod::GET => self.get_all_todos(),
            HttpMethod::POST => self.create_todo(request),
            HttpMethod::DELETE => self.delete_todo(request),
            HttpMethod::PUT => self.update_todo(request),
            _ => self.error_service.serve_400_response(String::from("Bad request"))
        }
    }
}

mod tests {
    use crate::http::request::Request;

    #[test]
    fn get_id_from_request() {
    }
}
