use crate::net::request::Request;
use crate::net::response::Response;

pub trait Controller {
    fn execute_request(&self, request: Request) -> Response;
}
