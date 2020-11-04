use postgres::{Client, NoTls};

pub struct UserService {
    logger: Logger,
    client: Client,
}

impl UserService {
    pub fn new() -> UserService {
        let logger = Logger::new(String::from("UserService"));
        let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

        UserService {
            logger: logger,
            conn: conn,
        }
    }

    pub fn create_user(&self, create_user_dto: CreateUserDTO) -> Result<User, Error> {
        self.client.execute(
            "INSERT INTO users (username, password) VALUES ($1, $2)",
            &[&create_user_dto.username, &author.password],
        )?;
        Ok()
    }

    pub fn update_user(&self, update_user_dto: UpdateUserDTO, user_id: u32) -> Result<User, Error> {
        self.client.execute(
            "UPDATE users u
                SET u.username = $1
             WHERE u.id = $2",
            &[&update_user_dto.username, $user_id],
        )?;
        Ok()
    }

    pub fn delete_user(&self, user_id: u32) -> Result<(), Error> {
        self.client.execute(
            "DELETE * FROM user u WHERE u.id = $1",
            &[&user_id],
        )?;
        Ok()
    }
}
