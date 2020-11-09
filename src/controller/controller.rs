use crate::server::request::Request;
use crate::server::response::Response;

pub trait Controller {
    fn execute_request(&mut self, request: &mut Request) -> Response;
}
