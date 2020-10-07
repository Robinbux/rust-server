use crate::enums::content_type::ContentType;

mod mime_response_constants {
    pub(crate) const LINEBREAK: &str = "\r\n";
    pub(crate) const DOUBLE_LINEBREAK: &str = "\n\n";
    pub(crate) const HTTP: &str = "HTTP/1.1 ";
    pub(crate) const CONTENT_TYPE: &str = "Content-Type: ";
    pub(crate) const CONTENT_LENGTH: &str ="Content-Length: ";
}

pub struct MimeResponse {
    pub http_status_code: String,
    pub content_type: ContentType,
    pub content: String,
}

impl MimeResponse {
    pub fn build_mime_response(&mut self) -> String {
        let content_length = self.content.len();
        format!(
            "\
        {http_status_code_constant}{http_status_code}{linebreak}\
        {content_type_constant}{content_type}{linebreak}\
        {content_length_constant}{constant_length}{double_linebreak}\
        {content}\
        ",
            linebreak = mime_response_constants::LINEBREAK,
            double_linebreak = mime_response_constants::DOUBLE_LINEBREAK,
            http_status_code_constant = mime_response_constants::HTTP,
            http_status_code = self.http_status_code,
            content_type_constant = mime_response_constants::CONTENT_TYPE,
            content_type = self.content_type.as_str(),
            content_length_constant = mime_response_constants::CONTENT_LENGTH,
            constant_length = content_length,
            content = self.content
        )
    }
}
