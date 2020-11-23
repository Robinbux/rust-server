use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::resource_service::ResourceService;
use crate::utils::logger::Logger;

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

    pub fn load_resource(&mut self, request: &Request) -> Response {
        self.resource_service.load_resource(&request)
    }
}

impl Controller for ResourcesController {
    fn execute_request(&mut self, request: &mut Request) -> Response {
        request.current_child_path = BaseController::extract_child_path(&request.resource_path);
        self.load_resource(request)
    }
}
