use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::resource_service::ResourceService;
use crate::utils::logger::Logger;

#[derive(Clone)]
pub struct ResourcesController {
    #[allow(dead_code)]
    logger: Logger,
    resource_service: ResourceService,
}

impl ResourcesController {
    pub fn new() -> ResourcesController {
        let logger = Logger::new(String::from("ResourcesController"));
        let resource_service = ResourceService::new();
        ResourcesController {
            logger,
            resource_service,
        }
    }

    pub fn serve_file(&self, file_path: &str) -> Response {
        ResourceService::serve_file(&self.resource_service.error_service, file_path)
    }
}

impl Controller for ResourcesController {
    fn execute_request(&self, mut request: Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        self.serve_file(&request.resource_path)
    }
}
