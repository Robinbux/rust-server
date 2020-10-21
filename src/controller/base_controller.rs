use crate::controller::admin_controller::AdminController;
use crate::controller::controller::Controller;
use crate::utils::logger::Logger;
use crate::enums::content_type::ContentType;
use crate::controller::assets_controller::AssetsController;
use crate::utils::file_handler::file_handler;

pub struct BaseController {
    #[allow(dead_code)]
    logger: Logger,
    admin_controller: AdminController,
    assets_controller: AssetsController,
}

impl BaseController {
    pub fn new() -> BaseController {
        let logger = Logger::new(String::from("BaseController"));
        let admin_controller = AdminController::new();
        let assets_controller = AssetsController::new();
        BaseController {
            logger: logger,
            admin_controller: admin_controller,
            assets_controller: assets_controller,
        }
    }

    fn serve_404_page(&self) -> Vec<u8> {
        Vec::from(&file_handler::load_resource(files::ERROR_404).expect("Unable to load resource")[..])
    }

    pub fn extract_parent_path(path: &str) -> &str {
        path.split("/").nth(1).expect("Unable to split result")
    }

    pub fn extract_child_path(path: &str) -> String {
        let &split = path.split("/");
        if split.count()< &3 {
            return String::from("")
        }
        let child_path = split.collect::<Vec<&str>>()[2..].join("/");

        format!("/{}", child_path)
    }
}

mod files {
    pub const ERROR_404: &str = "404.html";
}

impl Controller for BaseController {
    fn serve_content(&self, path: &str) -> Vec<u8> {
        let route_beginning = BaseController::extract_parent_path(path);
        let child_path = BaseController::extract_child_path(path);
        return match route_beginning {
            "admin" => self.admin_controller.serve_content(&child_path),
            "assets" => self.assets_controller.serve_content(&child_path),
            _ => self.serve_404_page(),
        };
    }

    fn get_content_type_for_path(&self, path: &str) -> ContentType {
        let route_beginning = BaseController::extract_parent_path(path);
        let child_path = BaseController::extract_child_path(path);
        return match route_beginning {
            "admin" => self.admin_controller.get_content_type_for_path(&child_path),
            "assets" => self.assets_controller.get_content_type_for_path(&child_path),
            _ => panic!("Base controller route not found"),
        };
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
