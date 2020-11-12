use crate::enums::content_type::ContentType;
use crate::enums::http_methods::HttpMethod;

pub struct Request {
    pub(crate) http_method: HttpMethod,
    pub(crate) resource_path: String,
    pub(crate) payload: Option<Vec<u8>>,
    #[allow(dead_code)]
    pub(crate) content_type: Option<ContentType>,
    pub(crate) current_child_path: String,
}

impl Request {
    pub fn new(request_string: String) -> Request {
        let http_method = HttpMethod::get_http_method(&request_string);
        let resource_path = Request::extract_resource_path(&request_string).to_string();
        let payload = Request::extract_payload(&request_string);
        let content_type = Request::extract_content_type(&request_string);
        let current_child_path = resource_path.clone();

        Request {
            http_method,
            resource_path,
            payload,
            content_type,
            current_child_path,
        }
    }

    fn extract_payload(request_string: &String) -> Option<Vec<u8>> {
        let result = request_string.split("\r\n\r\n").collect::<Vec<&str>>();
        if result.len() == 1 {
            return None;
        }
        let payload = result.last().expect("Unable to split result").as_bytes();
        Some(payload.to_vec())
    }

    fn extract_resource_path(request_string: &str) -> &str {
        request_string
            .split_whitespace()
            .nth(1)
            .expect("Unable to split result")
    }

    fn extract_content_type(request_string: &str) -> Option<ContentType> {
        let result = request_string
            .split("Content-Type: ")
            .collect::<Vec<&str>>();
        if result.len() == 1 {
            return None;
        }
        let content_type = result[1]
            .split_whitespace()
            .nth(0)
            .expect("Unable to split content type")
            .to_string();
        Some(ContentType::from_str(content_type).unwrap())
    }
}
