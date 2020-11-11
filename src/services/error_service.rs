use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::response::Response;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;
use serde::{Deserialize, Serialize};
use serde_json;

mod files {
    pub const ERROR_404: &str = "404.html";
}

pub struct ErrorService {
    #[allow(dead_code)]
    logger: Logger,
}

#[derive(Serialize, Deserialize)]
struct ErrorMessage {
    message: String,
}

impl ErrorService {
    pub fn new() -> ErrorService {
        let logger = Logger::new(String::from("ErrorService"));
        ErrorService { logger: logger }
    }

    pub fn serve_404_page(&self) -> Response {
        let content_bytes = Vec::from(
            &file_handler::load_resource(files::ERROR_404).expect("Unable to load resource")[..],
        );
        Response::new(content_bytes, ContentType::HTML, HTTPStatusCodes::NotFound)
    }

<<<<<<< HEAD
    pub fn serve_400_response(&mut self, error_message: String) -> Response {
        self.logger.log(&error_message);
        let json_message = ErrorMessage{message: error_message};
=======
    pub fn serve_400_response(&self, error_message: String) -> Response {
        let json_message = ErrorMessage {
            message: error_message,
        };
>>>>>>> fbf1afeedbb5ac9e38b68175af2b0d3501fd79b2
        let json_str = serde_json::to_string(&json_message).unwrap();
        let content_bytes: &[u8] = json_str.as_ref();
        let content_bytes = content_bytes.to_vec();
        Response::new(
            content_bytes,
            ContentType::JSON,
            HTTPStatusCodes::BadRequest,
        )
    }

<<<<<<< HEAD
    pub fn serve_500_response(&mut self, error_message: String) -> Response {
        self.logger.log(&error_message);
        let json_message = ErrorMessage{message: error_message};
=======
    pub fn serve_500_response(&self, error_message: String) -> Response {
        let json_message = ErrorMessage {
            message: error_message,
        };
>>>>>>> fbf1afeedbb5ac9e38b68175af2b0d3501fd79b2
        let json_str = serde_json::to_string(&json_message).unwrap();
        let content_bytes: &[u8] = json_str.as_ref();
        let content_bytes = content_bytes.to_vec();
        Response::new(
            content_bytes,
            ContentType::JSON,
            HTTPStatusCodes::InternalServerError,
        )
    }
}
