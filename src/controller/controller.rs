use crate::http::request::Request;
use crate::http::response::Response;

pub trait Controller {
    fn execute_request(&self, request: Request) -> Response;
}
