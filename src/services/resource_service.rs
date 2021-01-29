use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;

use crate::http::response::Response;
use crate::services::error_service::ErrorService;
use crate::utils::logger::Logger;

#[derive(Clone)]
pub struct ResourceService {
    #[allow(dead_code)]
    pub(crate) error_service: ErrorService,
    #[allow(dead_code)]
    logger: Logger,
}

impl ResourceService {
    pub fn new() -> ResourceService {
        let logger = Logger::new(String::from("ResourceService"));
        let error_service = ErrorService::new();
        ResourceService {
            error_service,
            logger,
        }
    }

    pub fn serve_file(error_service: &ErrorService, file_path: &str) -> Response {
        let file = ResourceService::load_from_file_path(file_path);
        if file.is_err() {
            return error_service.serve_404_page();
        }
        let content_type_result = ContentType::get_content_type_from_file_path(file_path);
        if content_type_result.is_err(){
            return error_service.serve_400_response(content_type_result.unwrap_err())
        }
        Response::new(file.unwrap(), content_type_result.unwrap(), HTTPStatusCodes::Ok)
    }

    pub fn load_resource(file_name: &str) -> Result<Vec<u8>, String> {
        let raw_file_name = ResourceService::extract_raw_file_name(file_name);
        let content_type = ContentType::get_content_type_from_file_path(&raw_file_name);
        if content_type.is_err(){
            return Err(content_type.unwrap_err());
        }
        match content_type.unwrap() {
            ContentType::HTML => Ok(ResourceService::load_from_file_path(&format!(
                "resources/html/{}",
                &raw_file_name
            ))
            .unwrap()),
            ContentType::ICO => Ok(ResourceService::load_from_file_path(&format!(
                "resources/assets/{}",
                &raw_file_name
            ))
            .unwrap()),
            ContentType::PNG => Ok(ResourceService::load_from_file_path(&format!(
                "resources/assets/{}",
                &raw_file_name
            ))
            .unwrap()),
            ContentType::JAVASCRIPT => Ok(ResourceService::load_from_file_path(&format!(
                "resources/js/{}",
                &raw_file_name
            ))
            .unwrap()),
            ContentType::CSS => Ok(ResourceService::load_from_file_path(&format!(
                "resources/css/{}",
                &raw_file_name
            ))
            .unwrap()),
            ContentType::MP4 => Ok(ResourceService::load_from_file_path(&format!(
                "resources/assets/{}",
                &raw_file_name
            ))
            .unwrap()),
            _ => Err("Unsupported content type!".to_string()),
        }
    }

    // If the provided file_name is a path eg. /css/style.css, we have to extract the raw file name.
    fn extract_raw_file_name(file_name: &str) -> &str {
        if !file_name.contains('/') {
            return file_name;
        }
        file_name.split('/').last().unwrap()
    }

    pub fn load_from_file_path(file_path: &str) -> tokio::io::Result<Vec<u8>> {
        std::fs::read(file_path)
    }
}

mod tests {
    use super::*;
    use crate::services::error_service::ErrorMessage;


    #[should_panic]
    #[test]
    fn load_resource_missing() {
        let _html = ResourceService::load_resource("test.html").unwrap();
    }

    #[test]
    fn load_resource_valid() {
        let html = ResourceService::load_resource("index.html").unwrap();
        let expected_html: Vec<u8> = std::fs::read("resources/html/index.html").unwrap();
        assert_eq!(expected_html, html)
    }

    #[test]
    fn serve_file_valid() {
        let error_service = ErrorService::new();
        let response = ResourceService::serve_file(&error_service, "resources/html/index.html");
        let html_bytes =
"<!DOCTYPE html>
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
</html>".as_bytes().to_vec();
        let expected_response = Response::new(html_bytes, ContentType::HTML, HTTPStatusCodes::Ok);
        assert_eq!(expected_response, response)
    }

    #[test]
    fn serve_file_errorneous() {
        let error_service = ErrorService::new();
        let error_response = ResourceService::serve_file(&error_service, "resources/index.html");
        let html_bytes =
"<!DOCTYPE html>
<html lang=\"en\">
<head>
    <title>404 Not Found</title>
</head>
<body>
<h1>404, resource not found</h1>

</body>
</html>".as_bytes().to_vec();
        let expected_response = Response::new(html_bytes, ContentType::HTML, HTTPStatusCodes::NotFound);
        assert_eq!(expected_response, error_response)
    }

    #[test]
    fn serve_file_incompatible_content_type() {

        let error_service = ErrorService::new();
        let error_response = ResourceService::serve_file(&error_service, "resources/logs/Log.txt");

        let error_message = String::from("Unable to convert given String to ContentType");
        let json_message = ErrorMessage {
            message: error_message,
        };
        let json_str = serde_json::to_string(&json_message).unwrap();
        let content_bytes: &[u8] = json_str.as_ref();
        let content_bytes= content_bytes.to_vec();
        let expected_response = Response::new(
            content_bytes,
            ContentType::JSON,
            HTTPStatusCodes::BadRequest,
        );
        assert_eq!(expected_response, error_response)
    }

    #[test]
    fn extract_raw_file_name() {
        let file_name = "/css/style.css";
        let result = ResourceService::extract_raw_file_name(file_name);
        let expected = "style.css";
        assert_eq!(expected, result)
    }

    #[test]
    fn extract_raw_file_name_empty() {
        let file_name = "";
        let result = ResourceService::extract_raw_file_name(file_name);
        let expected = "";
        assert_eq!(expected, result)
    }
}
