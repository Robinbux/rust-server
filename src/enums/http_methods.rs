#[derive(PartialEq, Debug, Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    UPDATE,
    PUT,
}

impl HttpMethod {
    pub fn get_http_method(request_string: &str) -> HttpMethod {
        let http_method = request_string
            .split_whitespace()
            .next()
            .expect("Unable to split result");
        match http_method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "DELETE" => HttpMethod::DELETE,
            "UPDATE" => HttpMethod::UPDATE,
            "PUT" => HttpMethod::PUT,
            _ => panic!("invalid HTTP method"),
        }
    }
}
