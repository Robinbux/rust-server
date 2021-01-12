use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::utils::logger::Logger;
use crate::services::resource_service::ResourceService;

#[derive(Clone)]
pub struct AssetsController {
    #[allow(dead_code)]
    logger: Logger,
    error_service: ErrorService,
    resource_service: ResourceService,
}

impl AssetsController {
    pub fn new() -> AssetsController {
        let logger = Logger::new(String::from("AssetsController"));
        let error_service = ErrorService::new();
        let resource_service = ResourceService::new();
        AssetsController {
            logger,
            error_service,
            resource_service,
        }
    }

    pub fn serve_pika(&self) -> Response {
        let bytes = ResourceService::load_resource(&self.error_service, "pikachu.png").expect("Unable to load resource");
        Response::new(bytes, ContentType::PNG, HTTPStatusCodes::Ok)
    }

    pub fn serve_mp4(&self) -> Response {
        let bytes = ResourceService::load_resource(&self.error_service, "sample_vid.mp4").expect("Unable to load resource");
        Response::new(bytes, ContentType::MP4, HTTPStatusCodes::Ok)
    }

    pub fn serve_fav_icon(&self) -> Response {
        let bytes = ResourceService::load_resource(&self.error_service,"favicon.ico").expect("Unable to load resource");
        Response::new(
            bytes,
            ContentType::ICO,
            HTTPStatusCodes::Ok)
    }
}

impl Controller for AssetsController {
    fn execute_request(&self, request: &mut Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        let route_beginning = BaseController::extract_parent_path(&request.current_child_path);
        match route_beginning {
            "pika" => self.serve_pika(),
            "" => self.serve_fav_icon(),
            "vid" => self.serve_mp4(),
            _ => self.error_service.serve_404_page(),
        }
    }
}
