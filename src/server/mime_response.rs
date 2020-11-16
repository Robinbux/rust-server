use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;

mod mime_response_constants {
    pub(crate) const LINEBREAK: &str = "\r\n";
    pub(crate) const HTTP: &str = "HTTP/1.1 ";
    pub(crate) const CONTENT_TYPE: &str = "Content-Type: ";
    pub(crate) const CONTENT_LENGTH: &str = "Content-Length: ";
}
#[derive(PartialEq, Debug)]
pub struct MimeResponse {
    pub http_status_code: HTTPStatusCodes,
    pub content_type: ContentType,
    pub content_length: usize,
}

impl MimeResponse {
    pub fn build_mime_response(&mut self) -> String {
        format!(
            "\
        {http_status_code_constant}{http_status_code}{linebreak}\
        {content_type_constant}{content_type}{linebreak}\
        {content_length_constant}{content_length}{linebreak}\
        {linebreak}\
        ",
            linebreak = mime_response_constants::LINEBREAK,
            http_status_code_constant = mime_response_constants::HTTP,
            http_status_code = self.http_status_code.as_str(),
            content_type_constant = mime_response_constants::CONTENT_TYPE,
            content_type = self.content_type.as_str(),
            content_length_constant = mime_response_constants::CONTENT_LENGTH,
            content_length = self.content_length,
        )
    }
}

mod test {
    #[cfg(test)]
    use super::*;

    #[test]
    fn build_mime_response_html() {
        let mut expected_mime_string = String::from(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 327\r\n\r\n",
        );

        let mime_response = MimeResponse {
            http_status_code: HTTPStatusCodes::Ok,
            content_type: ContentType::HTML,
            content_length: 327,
        }
        .build_mime_response();

        assert_eq!(expected_mime_string, mime_response)
    }

    #[test]
    fn build_mime_response_not_found() {
        let mut expected_mime_string = String::from(
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 327\r\n\r\n",
        );

        let mime_response = MimeResponse {
            http_status_code: HTTPStatusCodes::NotFound,
            content_type: ContentType::HTML,
            content_length: 327,
        }
        .build_mime_response();

        assert_eq!(expected_mime_string, mime_response)
    }

    #[test]
    fn build_mime_response_png() {
        let mut expected_mime_string = String::from(
            "HTTP/1.1 500 Internal Server Error\r\nContent-Type: image/png\r\nContent-Length: 327\r\n\r\n",
        );

        let mime_response = MimeResponse {
            http_status_code: HTTPStatusCodes::InternalServerError,
            content_type: ContentType::PNG,
            content_length: 327,
        }
        .build_mime_response();

        assert_eq!(expected_mime_string, mime_response)
    }
}
