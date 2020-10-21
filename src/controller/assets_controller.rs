use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;
use serde::Serialize;
use std::fs;

pub struct AssetsController {
    #[allow(dead_code)]
    logger: Logger,
}

impl AssetsController {
    pub fn new() -> AdminController {
        let logger = Logger::new(String::from("AssetsController"));
        AssetsController { logger: logger }
    }

    pub fn pika() -> String {
        "Test".to_string()
    }
}

impl Controller for AssetsController {
    fn serve_content(&self, path: &str) -> String {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "pika" => self.console(),
            _ => panic!("Unknown Path"),
        };
    }

    fn get_content_type_for_path(&self, path: &str) -> ContentType {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "console" => ContentType::HTML,
            _ => panic!("Unknown Path"),
        };
    }
}
