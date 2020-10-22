#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum HTTPStatusCodes {
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

    pub fn as_str(&self) -> &'static str {
        match *self {
            HTTPStatusCodes::Ok => "200 OK",
            HTTPStatusCodes::Created => "201 Created",
            HTTPStatusCodes::Accepted => "202 Accepted",
            HTTPStatusCodes::BadRequest => "400 Bad Request",
            HTTPStatusCodes::Unauthorized => "401 Unauthorized",
            HTTPStatusCodes::Forbidden => "403 Forbidden",
            HTTPStatusCodes::NotFound => "404 Not Found",
            HTTPStatusCodes::InternalServerError => "500 Internal Server Error",
        }
    }
}
