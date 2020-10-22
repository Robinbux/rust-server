use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::utils::file_handler::file_handler;
use crate::utils::logger::Logger;
use serde::Serialize;
use std::fs;
use image::ImageFormat;

pub struct AssetsController {
    #[allow(dead_code)]
    logger: Logger,
}

impl AssetsController {
    pub fn new() -> AssetsController {
        let logger = Logger::new(String::from("AssetsController"));
        AssetsController { logger: logger }
    }

    pub fn pika(&self) -> Vec<u8> {
        file_handler::load_resource("pikachu.png").expect("Unable to load resource")
    }

    pub fn serve_fav_icon() -> Vec<u8> {
        file_handler::load_resource("favicon.ico").expect("Unable to load resource")
    }
}

impl Controller for AssetsController {
    fn serve_content(&self, path: &str) -> Result<Vec<u8>, Vec<u8>>{
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "pika" => Ok(self.pika()),
            "favicon.ico" => Ok(AssetsController::serve_fav_icon()),
            _ => Err(BaseController::serve_404_page()),
        };
    }

    fn get_content_type_for_path(&self, path: &str) -> ContentType {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "pika" => ContentType::PNG,
            "favicon.ico" => ContentType::ICO,
            _ => ContentType::HTML,
        };
    }
}
