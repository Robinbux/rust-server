use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::response::Response;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;

mod files {
    pub const ERROR_404: &str = "404.html";
}

pub struct ErrorService {
    #[allow(dead_code)]
    logger: Logger,
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
}
