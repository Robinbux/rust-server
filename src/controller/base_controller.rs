use crate::controller::admin_controller::AdminController;
use crate::controller::controller::Controller;
use crate::utils::logger::Logger;

pub struct BaseController {
    #[allow(dead_code)]
    logger: Logger,
    admin_controller: AdminController,
}

impl BaseController {
    pub fn new() -> BaseController {
        let logger = Logger::new(String::from("BaseController"));
        let admin_controller = AdminController::new();
        BaseController {
            logger: logger,
            admin_controller: admin_controller,
        }
    }

    pub fn serve_content(&self, path: &str) -> String {
        let route_beginning = BaseController::extract_parent_path(path);
        let child_path = BaseController::extract_child_path(path);
        assert_eq!(route_beginning, "admin");
        return match route_beginning {
            "admin" => self.admin_controller.serve_content(&child_path),
            _ => panic!("Base controller route not found"),
        };
    }

    pub fn extract_parent_path(path: &str) -> &str {
        path.split("/").nth(1).expect("Unable to split result")
    }

    pub fn extract_child_path(path: &str) -> String {
        path.split("/").collect::<Vec<&str>>()[1..].join("/")
    }
}

mod tests {
    use super::BaseController;
    use super::*;
    #[cfg(test)]
    #[test]
    fn extract_child_path() {
        let path = "admin/console/index";
        let result = BaseController::extract_child_path(path);
        assert_eq!("console/index", result)
    }

    #[test]
    fn extract_child_path_empty() {
        let path = "admin/";
        let result = BaseController::extract_child_path(path);
        assert_eq!("", result)
    }
}
