use crate::enums::content_type::ContentType;
use crate::enums::http_status_codes::HTTPStatusCodes;
use crate::server::request::Request;
use crate::server::response::Response;
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
        let file = ResourceService::load_from_file_path(String::from(file_path));
        if file.is_err() {
            return error_service
                .serve_404_page()
        }
        let content_type = ContentType::get_content_type_from_file_path(file_path).unwrap();
        Response::new(file.unwrap(), content_type, HTTPStatusCodes::Ok)
    }

    pub fn load_resource(error_service: &ErrorService, file_name: &str) -> Result<Vec<u8>, String> {
        let raw_file_name = ResourceService::extract_raw_file_name(file_name);
        let content_type = ContentType::get_content_type_from_file_path(&raw_file_name).expect("incompatible content type");
        match content_type {
            ContentType::HTML => Ok(ResourceService::load_from_file_path(format!("resources/html/{}", &raw_file_name)).unwrap()),
            ContentType::ICO => Ok(ResourceService::load_from_file_path(format!("resources/assets/{}", &raw_file_name)).unwrap()),
            ContentType::PNG => Ok(ResourceService::load_from_file_path(format!("resources/assets/{}", &raw_file_name)).unwrap()),
            ContentType::JAVASCRIPT => Ok(ResourceService::load_from_file_path(format!("resources/js/{}", &raw_file_name)).unwrap()),
            ContentType::CSS => Ok(ResourceService::load_from_file_path(format!("resources/css/{}", &raw_file_name)).unwrap()),
            ContentType::MP4 => Ok(ResourceService::load_from_file_path(format!("resources/assets/{}", &raw_file_name)).unwrap()),
            _ => panic!("Unsupported content type!"),
        }
    }

    // If the provided file_name is a path eg. /css/style.css, we have to extract the raw file name.
    fn extract_raw_file_name(file_name: &str) -> String {
        if !file_name.contains("/") {
            return file_name.to_owned();
        }
        file_name.split("/").last().unwrap().to_owned()
    }

    pub fn load_from_file_path(file_path: String) -> tokio::io::Result<Vec<u8>> {
        let bytes = std::fs::read(file_path);
        bytes
    }
}

mod tests {
    use crate::services::resource_service::ResourceService;
    use crate::services::error_service::ErrorService;
    use crate::utils::logger::Logger;

    const ERROR_FILE_PATH: &str = "resources/html/404.html";

    #[test]
    fn load_resource_erroneous() {
        let error_file: Vec<u8> = std::fs::read(ERROR_FILE_PATH).unwrap();
        let error_service = ErrorService.new();
        let html = load_resource(error_service, "test.html").unwrap();
        assert_eq!(error_file, html)
    }

    #[test]
    fn load_resource_valid() {
        let html = load_resource("index.html").unwrap();
        let expected_html: Vec<u8> = std::fs::read("resources/html/index.html").unwrap();
        assert_eq!(expected_html, html)
    }
}

