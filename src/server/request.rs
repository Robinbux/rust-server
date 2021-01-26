use crate::enums::content_type::ContentType;
use crate::enums::http_methods::HttpMethod;

pub struct Request {
    pub(crate) http_method: HttpMethod,
    pub(crate) resource_path: String,
    pub(crate) payload: Option<String>,
    pub(crate) content_type: Option<ContentType>,
    pub(crate) current_child_path: String,
}

impl Request {
    pub fn new(request_string: String) -> Request {
        let http_method = HttpMethod::get_http_method(&request_string);
        let resource_path = Request::extract_resource_path(&request_string);
        let content_type = Request::extract_content_type(&request_string);
        let payload = Request::extract_payload(request_string);
        let current_child_path = resource_path.clone();

        Request {
            http_method,
            resource_path,
            payload,
            content_type,
            current_child_path,
        }
    }

    fn extract_payload(request_string: String) -> Option<String> {
        let index = request_string.find("\r\n\r\n");
        let end_index = request_string.rfind('}');
        if index.is_some() & end_index.is_some(){
            return Some(request_string[index.unwrap()+4..end_index.unwrap()+1].parse().unwrap());
        }
        None
    }

    fn extract_resource_path(request_string: &str) -> String {
        request_string
            .split_whitespace()
            .nth(1)
            .unwrap()
            .to_string()
            .split_off(1)
    }

    fn extract_content_type(request_string: &str) -> Option<ContentType> {
        let result = request_string
            .split("Content-Type: ")
            .collect::<Vec<&str>>();
        if result.len() == 1 {
            return None;
        }
        let content_type_string = result[1]
            .split_whitespace()
            .next()
            .unwrap();
        let content_type_result = ContentType::from_str(content_type_string);
        if content_type_result.is_err() {
            return None;
        }
        Some(content_type_result.unwrap())
    }
}

mod tests {

    use super::*;

    #[test]
    fn extract_payload() {
        let request_string =
            String::from("GET /favicon.ico HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"todo_message\":\"pee\"}");

        let payload = Request::extract_payload(request_string).unwrap();
        let expected_payload = String::from("{\"todo_message\":\"pee\"}");
        assert_eq!(payload, expected_payload);
    }

    #[test]
    fn extract_payload_flawed_json() {
        let request_string =
            String::from("GET /favicon.ico HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"todo_message\":\"pee\"");
        let payload = Request::extract_payload(request_string);
        assert!(payload.is_none())
    }

    #[test]
    fn extract_payload_missing_newline() {
        let request_string =
            String::from("GET /favicon.ico HTTP/1.1\r\nHost: localhost:8087\r\n\r{\"todo_message\":\"pee\"}");
        let payload = Request::extract_payload(request_string);
        assert!(payload.is_none())
    }

    #[test]
    fn extract_resource_path() {
        let request_string = "GET /favicon.ico HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"todo_message\":\"pee\"}";
        let result = Request::extract_resource_path(request_string);
        assert_eq!(result, String::from("favicon.ico"))
    }

    #[test]
    fn extract_resource_path_long() {
        let request_string = "GET /resources/assets/pikachu.png HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"todo_message\":\"pee\"}";
        let result = Request::extract_resource_path(request_string);
        assert_eq!(result, String::from("resources/assets/pikachu.png"))
    }

    #[test]
    fn extract_resource_path_empty() {
        let request_string = "GET / HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"todo_message\":\"pee\"}";
        let result = Request::extract_resource_path(request_string);
        assert_eq!(result, String::from(""))
    }

    #[test]
    fn extract_content_type_missing() {
        let request_string = "GET / HTTP/1.1\r\nHost: localhost:8087\r\n\r\n{\"todo_message\":\"pee\"}";
        let result = Request::extract_content_type(request_string);
        assert_eq!(result, None)
    }

    #[test]
    fn extract_content_type_json() {
        let request_string = "POST / HTTP/1.1\r\nHost: localhost:8087\r\nContent-Type: application/json; charset=utf-8\r\n\r\n{\"todo_message\":\"pee\"}";
        let result = Request::extract_content_type(request_string);
        assert_eq!(result.unwrap(), ContentType::JSON)
    }
}
