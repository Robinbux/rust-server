use crate::enums::content_type::ContentType;
use crate::server::request::Request;

pub trait Controller {
    fn execute_request(&self, request: Request) -> Result<Vec<u8>, Vec<u8>>;
    fn get_content_type_for_path(&self, path: &str) -> ContentType;
}
