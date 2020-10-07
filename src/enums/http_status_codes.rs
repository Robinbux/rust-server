enum HTTPStatusCodes {
    OK,
    CREATED,
    ACCEPTED,
    BAD_REQUEST,
    UNAUTHORIZED,
    FORBIDDEN,
    NOT_FOUND,
    INTERNAL_SERVER_ERROR
}

impl HTTPStatusCodes {
    pub fn as_code_value(&self) -> u16 {
        match *self {
            HTTPStatusCodes::OK => 200,
            HTTPStatusCodes::CREATED => 201,
            HTTPStatusCodes::ACCEPTED => 202,
            HTTPStatusCodes::BAD_REQUEST => 400,
            HTTPStatusCodes::UNAUTHORIZED => 401,
            HTTPStatusCodes::FORBIDDEN => 403,
            HTTPStatusCodes::NOT_FOUND => 404,
            HTTPStatusCodes::INTERNAL_SERVER_ERROR => 500,
        }
    }
}