pub struct CreateUserDTO {
    pub(crate) username: String,
    pub(crate) password: String,
}

pub struct UpdateUserDTO {
    pub(crate) username: String,
}

pub struct UserDTO {
    user_name: String,
    password: String,
    id: u32,
}
