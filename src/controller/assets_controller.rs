use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::net::request::Request;
use crate::net::response::Response;
use crate::services::error_service::ErrorService;
use crate::services::resource_service::ResourceService;
use crate::utils::logger::Logger;

#[derive(Clone)]
pub struct AssetsController {
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
        let result = ResourceService::load_resource("pikachu.png");
        if result.is_err(){
            return self.error_service.serve_400_response(result.unwrap_err())
        }
        Response::new(result.unwrap(), ContentType::PNG, HTTPStatusCodes::Ok)
    }

    pub fn serve_mp4(&self) -> Response {
        let result = ResourceService::load_resource( "sample_vid.mp4");
        if result.is_err(){
            return self.error_service.serve_400_response(result.unwrap_err())
        }
        Response::new(result.unwrap(), ContentType::MP4, HTTPStatusCodes::Ok)
    }

    pub fn serve_fav_icon(&self) -> Response {
        let result = ResourceService::load_resource("favicon.ico");
        if result.is_err(){
            return self.error_service.serve_400_response(result.unwrap_err())
        }
        Response::new(result.unwrap(), ContentType::ICO, HTTPStatusCodes::Ok)
    }
}

impl Controller for AssetsController {
    fn execute_request(&self, mut request: Request) -> Response {
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
