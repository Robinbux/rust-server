pub struct CreateUserDTO {
    username: String,
    password: String,
}

pub struct UpdateUserDTO {
    username: String,
}

pub struct UserDTO {
    user_name: String,
    password: String,
    id: u32,
}
