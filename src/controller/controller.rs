use crate::enums::content_type::ContentType;

pub trait Controller {
    fn serve_content(&self, path: &str) -> Result<Vec<u8>, Vec<u8>>;
    fn get_content_type_for_path(&self, path: &str) -> ContentType;
}
