pub struct UserService {
    logger: Logger,
}

impl UserService {
    pub fn new() -> UserService {
        let logger = Logger::new(String::from("UserService"));
        UserService { logger: logger }
    }

    pub fn createUser(&self, user_dto: UserDTO) -> 
}