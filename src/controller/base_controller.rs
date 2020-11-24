use crate::controller::admin_controller::AdminController;
use crate::controller::assets_controller::AssetsController;
use crate::controller::controller::Controller;
use crate::controller::home_controller::HomeController;
use crate::controller::notes_controller::NotesController;
use crate::controller::resources_controller::ResourcesController;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::services::error_service::ErrorService;
use crate::utils::logger::Logger;

#[derive(Clone)]
pub struct BaseController {
    #[allow(dead_code)]
    logger: Logger,
    admin_controller: AdminController,
    assets_controller: AssetsController,
    error_service: ErrorService, //user_controller: UserController,
    notes_controller: NotesController,
    home_controller: HomeController,
    resources_controller: ResourcesController,
}

impl BaseController {
    pub fn new() -> BaseController {
        let logger = Logger::new(String::from("BaseController"));
        let admin_controller = AdminController::new();
        let assets_controller = AssetsController::new();
        let error_service = ErrorService::new();
        let notes_controller = NotesController::new();
        let home_controller = HomeController::new();
        let resources_controller = ResourcesController::new();
        //let user_controller = UserController::new();
        BaseController {
            logger,
            admin_controller,
            assets_controller,
            error_service,
            notes_controller, //user_controller: user_controller,
            home_controller,
            resources_controller,
        }
    }

    pub fn extract_parent_path(path: &str) -> &str {
        let result = path.split("/").collect::<Vec<&str>>();
        if result.len() == 1 {
            return result[0];
        }
        result[1]
    }

    pub fn extract_child_path(path: &str) -> String {
        let split: Vec<&str> = path.split("/").collect();

        if split.len() < 3 {
            return String::from("/");
        }
        let child_path = split[2..].join("/");

        format!("/{}", child_path)
    }
}

impl Controller for BaseController {
    fn execute_request(&self, request: &mut Request) -> Response {
        let route_beginning = BaseController::extract_parent_path(&request.resource_path);
        return match route_beginning {
            "admin" => self.admin_controller.execute_request(request),
            "assets" => self.assets_controller.execute_request(request),
            "favicon.ico" => self.assets_controller.execute_request(request),
            "notes" => self.notes_controller.execute_request(request),
            "home" => self.home_controller.execute_request(request),
            "resources" => self.resources_controller.execute_request(request),
            //"user" => self.user_controller.execute_request(),
            _ => self.error_service.serve_404_page(),
        };
    }
}

#[allow(dead_code)]
mod tests {

    #[cfg(test)]
    #[test]
    fn extract_child_path() {
        let path = "/admin/console/index";
        let result = BaseController::extract_child_path(path);
        assert_eq!("/console/index", result)
    }

    #[test]
    fn extract_child_path_empty() {
        let path = "/admin/";
        let result = BaseController::extract_child_path(path);
        assert_eq!("/", result)
    }

    #[test]
    fn extract_child_path_missing() {
        let path = "/admin";
        let result = BaseController::extract_child_path(path);
        assert_eq!("", result)
    }

    #[test]
    fn extract_parent_path() {
        let path = "/admin/console/index";
        let result = BaseController::extract_parent_path(path);
        assert_eq!("admin", result)
    }

    #[test]
    fn extract_parent_path_empty() {
        let path = "/";
        let result = BaseController::extract_parent_path(path);
        assert_eq!("", result)
    }
}
