use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::response::Response;
use crate::services::resource_service;
use crate::utils::logger::Logger;
use serde::{Deserialize, Serialize};
use serde_json;

mod files {
    pub(crate) const ERROR_404_FILE_PATH: &str = "resources/html/404.html";
}

#[derive(Clone)]
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
        let content_bytes = resource_service::ResourceService::load_from_file_path(files::ERROR_404_FILE_PATH)
        .unwrap();
        Response::new(content_bytes, ContentType::HTML, HTTPStatusCodes::NotFound)
    }

    pub fn serve_400_response(&self, error_message: String) -> Response {
        self.logger.log(&error_message);
        let json_message = ErrorMessage {
            message: error_message,
        };
        let json_str = serde_json::to_string(&json_message).unwrap();
        let content_bytes: &[u8] = json_str.as_ref();
        let content_bytes = content_bytes.to_vec();
        Response::new(
            content_bytes,
            ContentType::JSON,
            HTTPStatusCodes::BadRequest,
        )
    }

    pub fn serve_500_response(&self, error_message: String) -> Response {
        self.logger.log(&error_message);
        let json_message = ErrorMessage {
            message: error_message,
        };
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
