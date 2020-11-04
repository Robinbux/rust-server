use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::dtos::user_dto::{CreateUserDTO, UpdateUserDTO};
use crate::enums::content_type::ContentType;
use crate::enums::http_methods::HttpMethod;
use crate::server::request::Request;
use crate::services::user_service::UserService;
use crate::utils::logger::Logger;
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub struct UserController {
    logger: Logger,
    user_service: UserService,
}

impl UserController {
    pub fn new() -> UserController {
        let logger = Logger::new(String::from("UserController"));
        let user_service = UserService::new();
        UserController {
            logger: logger,
            user_service: UserService,
        }
    }

    pub fn create_user(&self, create_user_payload: String) -> Result<Vec<u8>, Vec<u8>> {
        let dto: CreateUserDTO = serde_json::from_str(create_user_payload)?;
        self.user_service.create_user(dto);
    }

    pub fn update_user(
        &self,
        update_user_payload: String,
        user_id: u32,
    ) -> Result<Vec<u8>, Vec<u8>> {
        let dto: UpdateUserDTO = serde_json::from_str(update_user_payload)?;
        self.user_service.updateUser(dto);
    }

    pub fn delete_user(&self, user_id: u32) -> Result<Vec<u8>, Vec<u8>> {
        let dto: CreateUserDTO = serde_json::from_str(data)?;
        self.user_service.delete_user(user_id);
    }

    pub fn extract_user_id(path: String) -> String {
        path.split("/").last()
    }
}

impl Controller for UserController {
    fn execute_request(&self, request: Request) -> Result<Vec<u8>, Vec<u8>> {
        return match request.http_method {
            HttpMethod::POST => self.create_user(request.payload),
            HttpMethod::PUT => self.update_user(
                request.payload,
                UserController::extract_user_id(request.resource_path),
            ),
            HttpMethod::DELETE => {
                self.delete_user(UserController::extract_user_id(request.resource_path))
            }
            _ => Err(BaseController::serve_404_page()),
        };
    }

    fn get_content_type_for_path(&self, path: &str) -> ContentType {
        let route_beginning = BaseController::extract_parent_path(&path);
        return match route_beginning {
            "console" => ContentType::HTML,
            _ => ContentType::HTML,
        };
    }
}
