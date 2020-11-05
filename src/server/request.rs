use crate::enums::content_type::ContentType;
use crate::enums::http_methods::HttpMethod;
use regex::Regex;

pub struct Request {
    pub(crate) http_method: HttpMethod,
    pub(crate) resource_path: String,
    pub(crate) payload: String,
    pub(crate) content_type: ContentType,
    pub(crate) current_child_path: String,
}

impl Request {
    pub fn new(request_string: &str) -> Request {
        let http_method = HttpMethod::get_http_method(request_string);
        let resource_path = Request::extract_resource_path(request_string).to_string();
        let payload = Request::extract_payload(request_string);
        let content_type = Request::extract_content_type(request_string).unwrap();
        let current_child_path = resource_path.clone();

        Request {
            http_method,
            resource_path,
            payload,
            content_type,
            current_child_path,
        }
    }

    fn extract_payload(request_string: &str) -> String {
        request_string
            .split("\r\n")
            .last()
            .expect("Unable to split result")
            .to_string()
    }

    fn extract_resource_path(request_string: &str) -> &str {
        request_string
            .split_whitespace()
            .nth(1)
            .expect("Unable to split result")
    }

    fn extract_content_type(request_string: &str) -> Result<ContentType, ()> {
        let re = Regex::new("application/json").unwrap();
        let captures = re.captures(request_string).unwrap();
        let content_type: String = captures[0].to_string();
        ContentType::from_str(content_type)
    }
}
