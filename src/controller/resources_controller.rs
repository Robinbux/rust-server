use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;

#[derive(Clone)]
pub struct ResourcesController {
    #[allow(dead_code)]
    logger: Logger,
    error_service: ErrorService,
}

impl ResourcesController {
    pub fn new() -> ResourcesController {
        let logger = Logger::new(String::from("ResourcesController"));
        let error_service = ErrorService::new();
        ResourcesController {
            logger,
            error_service,
        }
    }

    pub fn load_resource(&self, request: &Request) -> Response {
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

impl Controller for ResourcesController {
    fn execute_request(&self, request: &mut Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        self.load_resource(request)
    }
}
