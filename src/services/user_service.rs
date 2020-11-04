use postgres::{Client, NoTls, Error};
use crate::utils::logger::Logger;
use crate::dtos::user_dto;

pub struct UserService {
    logger: Logger,
    client: Client,
}

impl UserService {
    pub fn new() -> UserService {
        let logger = Logger::new(String::from("UserService"));
        let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

        UserService {
            logger,
            client
        }
    }

    pub fn create_user(&mut self, create_user_dto: user_dto::CreateUserDTO) -> Result<u64, Error> {
        self.client.execute(
            "INSERT INTO users (username, password) VALUES ($1, $2)",
            &[&create_user_dto.username, &create_user_dto.password],
        )?;
    }

    pub fn update_user(&mut self, update_user_dto: user_dto::UpdateUserDTO, user_id: u32) -> Result<u64, Error> {
        self.client.execute(
            "UPDATE users u
                SET u.username = $1
             WHERE u.id = $2",
            &[&update_user_dto.username, user_id],
        )?;
    }

    pub fn delete_user(&mut self, user_id: u32) -> Result<u64, Error> {
        self.client.execute(
            "DELETE * FROM user u WHERE u.id = $1",
            &[&user_id],
        )?;
    }
}
