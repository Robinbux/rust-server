use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;

mod files {
    pub const HOME: &str = "home.html";
}


pub struct HomeController {
    #[allow(dead_code)]
    logger: Logger,
    error_service: ErrorService,
}

impl HomeController {
    pub fn new() -> HomeController {
        let logger = Logger::new(String::from("HomeController"));
        let error_service = ErrorService::new();
        HomeController {
            logger,
            error_service,
        }
    }

    pub fn home_page(&self) -> Response {
        let content_bytes = Vec::from(
            &file_handler::load_resource(files::HOME).expect("Unable to load resource")[..],
        );
        Response::new(content_bytes, ContentType::HTML, HTTPStatusCodes::Ok)
    }
}

impl Controller for HomeController {
    fn execute_request(&mut self, request: &mut Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        let route_beginning = BaseController::extract_parent_path(&request.current_child_path);
        match route_beginning {
            "" => self.home_page(),
            _ => self.error_service.serve_404_page(),
        }
    }
}
