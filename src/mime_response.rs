use crate::enums::content_type::ContentType;

mod mime_response_constants {
    pub(crate) const LINEBREAK: &str = "\r\n";
    pub(crate) const HTTP: &str = "HTTP/1.1 ";
    pub(crate) const CONTENT_TYPE: &str = "Content-Type: ";
    pub(crate) const CONTENT_LENGTH: &str = "Content-Length: ";
}
#[derive(PartialEq, Debug)]
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
        {content_length_constant}{content_length}{linebreak}\
        {linebreak}\
        {content}\
        ",
            linebreak = mime_response_constants::LINEBREAK,
            http_status_code_constant = mime_response_constants::HTTP,
            http_status_code = self.http_status_code,
            content_type_constant = mime_response_constants::CONTENT_TYPE,
            content_type = self.content_type.as_str(),
            content_length_constant = mime_response_constants::CONTENT_LENGTH,
            content_length = content_length,
            content = self.content
        )
    }
}

mod test {
    use super::*;

    #[test]
    fn build_mime_response_index_html() {

        let html_string = String::from(
            "\
            <!DOCTYPE html>
            <html lang=\"en\">
            <head>
                <meta charset=\"UTF-8\">
                <title>Hellooooooooo</title>
            </head>
            <body>
            <!DOCTYPE html>
            <h1>Cool Heading!</h1>
            <p>Awesome Paragraph!</p>
            </body>
            </html>",
        );
        let mut expected_mime_string = String::from("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 327\r\n\r\n");
        expected_mime_string.push_str(&html_string);

        let mime_response = MimeResponse {
            http_status_code: String::from("200 OK"),
            content_type: ContentType::HTML,
            content: html_string,
        }
        .build_mime_response();

        assert_eq!(expected_mime_string, mime_response)
    }
}
