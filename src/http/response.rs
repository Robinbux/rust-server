use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;

#[derive(Debug)]
pub struct Response {
    pub content_bytes: Vec<u8>,
    pub content_type: ContentType,
    pub http_status_code: HTTPStatusCodes,
}

impl Response {
    pub fn new(
        content_bytes: Vec<u8>,
        content_type: ContentType,
        http_status_code: HTTPStatusCodes,
    ) -> Response {
        Response {
            content_bytes,
            content_type,
            http_status_code,
        }
    }
}

impl PartialEq for Response {
    fn eq(&self, other: &Self) -> bool {
        (self.content_bytes == other.content_bytes) & (self.content_type==other.content_type) & (self.http_status_code==other.http_status_code)
    }
}
