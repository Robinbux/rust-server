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
pub struct AssetsController {
    #[allow(dead_code)]
    logger: Logger,
    error_service: ErrorService,
}

impl AssetsController {
    pub fn new() -> AssetsController {
        let logger = Logger::new(String::from("AssetsController"));
        let error_service = ErrorService::new();
        AssetsController {
            logger,
            error_service,
        }
    }

    pub fn pika(&self) -> Vec<u8> {
        file_handler::load_resource("pikachu.png").expect("Unable to load resource")
    }

    pub fn serve_fav_icon() -> Vec<u8> {
        file_handler::load_resource("favicon.ico").expect("Unable to load resource")
    }
}

impl Controller for AssetsController {
    fn execute_request(&self, request: &mut Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        let route_beginning = BaseController::extract_parent_path(&request.current_child_path);
        match route_beginning {
            "pika" => Response::new(self.pika(), ContentType::PNG, HTTPStatusCodes::Ok),
            "favicon.ico" => Response::new(
                AssetsController::serve_fav_icon(),
                ContentType::ICO,
                HTTPStatusCodes::Ok,
            ),
            _ => self.error_service.serve_404_page(),
        }
    }
}
