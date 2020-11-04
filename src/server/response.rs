use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;

pub struct Response {
    pub(crate) content_bytes: Vec<u8>,
    pub(crate) content_type: ContentType,
    pub(crate) http_status_code: HTTPStatusCodes,
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
