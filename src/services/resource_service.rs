use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;

pub struct ResourceService {
    #[allow(dead_code)]
    error_service: ErrorService,
    #[allow(dead_code)]
    logger: Logger,
}

impl ResourceService {
    pub fn new() -> ResourceService {
        let logger = Logger::new(String::from("ResourceService"));
        let error_service = ErrorService::new();
        ResourceService {
            error_service,
            logger,
        }
    }

    pub fn load_resource(&mut self, request: &Request) -> Response {
        let content_result = file_handler::load_resource(&request.current_child_path);
        if content_result.is_err() {
            return self
                .error_service
                .serve_400_response(String::from("Resource not found!"));
        }
        let content_type =
            ContentType::get_content_type_from_file_path(&request.current_child_path);
        Response::new(content_result.unwrap(), content_type, HTTPStatusCodes::Ok)
    }
}
