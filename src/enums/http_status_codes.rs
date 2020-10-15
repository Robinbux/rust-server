#[allow(dead_code)]
enum HTTPStatusCodes {
    Ok,
    Created,
    Accepted,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl HTTPStatusCodes {
    #[allow(dead_code)]
    pub fn as_code_value(&self) -> u16 {
        match *self {
            HTTPStatusCodes::Ok => 200,
            HTTPStatusCodes::Created => 201,
            HTTPStatusCodes::Accepted => 202,
            HTTPStatusCodes::BadRequest => 400,
            HTTPStatusCodes::Unauthorized => 401,
            HTTPStatusCodes::Forbidden => 403,
            HTTPStatusCodes::NotFound => 404,
            HTTPStatusCodes::InternalServerError => 500,
        }
    }
}
