#[allow(dead_code)]
pub struct CreateUserDTO {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[allow(dead_code)]
pub struct UpdateUserDTO {
    pub(crate) username: String,
}

#[allow(dead_code)]
pub struct UserDTO {
    user_name: String,
    password: String,
    id: u32,
}
