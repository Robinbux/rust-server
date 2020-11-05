use crate::enums::content_type::ContentType;
use crate::server::request::Request;
use crate::server::response::Response;

pub trait Controller {
    fn execute_request(&self, request: &mut Request) -> Response;
    fn get_content_type_for_path(&self, path: &str) -> ContentType;
}
