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
    pub fn new(buffer: Vec<u8>) -> Request {
        let request_string: String;
        request_string = String::from_utf8_lossy(buffer.as_slice())
            .parse()
            .expect("Parsing Failed");
        let http_method = HttpMethod::get_http_method(&request_string);
        let resource_path = Request::extract_resource_path(&request_string).to_string();
        let payload = Request::extract_payload(&buffer);
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

    fn extract_payload(buffer: &Vec<u8>) -> Option<Vec<u8>> {
        let index = Request::find_payload_index(&buffer);
        if let Some(index) = index {
            let test_str = std::str::from_utf8(&buffer[index + 4..]);
            return Some(buffer[index + 4..].to_vec());
        }
        None
    }

    fn find_payload_index(buffer: &[u8]) -> Option<usize> {
        buffer
            .windows(4)
            .enumerate()
            .find(|(_, w)| matches!(*w, b"\r\n\r\n"))
            .map(|(i, _)| i)
    }

    /*
    let result = buffer.split("\r\n\r\n".as_bytes());
        let test_one = "Das ist ein Test";
        let test_two =  String::from("Das ist ein Test");
        let str_request = std::str::from_utf8(request_string.as_ref());
        if result.len() == 1 {
            return None;
        }
        let payload = result.last().expect("Unable to split result");
        Some(payload.to_vec())
     */

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
        let content_type_result = ContentType::from_str(content_type);
        Some(content_type_result.unwrap())
    }
}
