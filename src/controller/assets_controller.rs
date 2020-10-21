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
        let bytes: Vec<u8> = std::fs::read("resources/assets/pikachu.png").unwrap();
        match image::load_from_memory_with_format(&bytes, ImageFormat::Png) {
            Ok(img) => {
                println!("input in png");
            }
            Err(_) => {
                println!("input is not png");
            }
        }
        bytes
    }

    pub fn serve_fav_icon() -> Vec<u8> {
        let bytes: Vec<u8> = std::fs::read("resources/assets/favicon.ico").unwrap();
        match image::load_from_memory_with_format(&bytes, ImageFormat::Ico) {
            Ok(img) => {
                println!("input in png");
            }
            Err(_) => {
                println!("input is not png");
            }
        }
        bytes
    }
}

impl Controller for AssetsController {
    fn serve_content(&self, path: &str) -> Vec<u8> {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "pika" => self.pika(),
            "favicon.ico" => AssetsController::serve_fav_icon(),
            _ => BaseController::serve_404_page(),
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
