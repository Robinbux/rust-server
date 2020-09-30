pub mod utils {
    use crate::content_type::ContentType;
    use std::fs::read_to_string;
    use std::fs;
    use crate::utils::utils;

    pub fn load_resource(file_path: String) -> Result<String, String> {
        let complete_resource_path = format!("resources{}", file_path);
        let content_type = ContentType::get_content_type_from_file_path(file_path);
        return match content_type {
            ContentType::HTML => Ok(utils::load_html(complete_resource_path)),
            ContentType::ICO => Ok(utils::load_ico(complete_resource_path)),
        }
    }

    fn load_html(html_file_path: String) -> String {
        return read_to_string(html_file_path).expect("Unable to read file to String");
    }

    fn load_ico(ico_file_path: String) -> String {
        let icon = fs::read(ico_file_path).expect("Unable to read icon");
        return base64::encode(&*icon);
    }
}