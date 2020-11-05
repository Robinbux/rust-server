#[allow(dead_code)]
use crate::services::user_service::UserService;
use crate::utils::logger::Logger;
#[allow(dead_code)]
pub struct UserController {
    logger: Logger,
    user_service: UserService,
}
#[allow(dead_code)]
impl UserController {
    /* pub fn new() -> UserController {
        let logger = Logger::new(String::from("UserController"));
        let user_service = UserService::new();
        UserController {
            logger,
            user_service
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

    pub fn delete_user(&mut self, user_id: u32) -> Result<Vec<u8>, Vec<u8>> {
        self.user_service.delete_user(user_id);
    }

    pub fn extract_user_id(path: String) -> u32 {
        path.split("/").last()
    } */
}

/* impl Controller for UserController {
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
 */
