user crate::dtos::userDto

pub struct UserController {
    logger: Logger,
}

impl UserController {
    pub fn new() -> UserController {
        let logger = Logger::new(String::from("UserController"));
        UserController { logger: logger }
    }

    pub fn create_user(request, userDto: ) {
        return crate::services
    }

    pub fn delete_user() {
    }

    pub fn update_user() {

    }
}