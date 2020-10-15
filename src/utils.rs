pub mod utils {
    use crate::enums::content_type::ContentType;
    use crate::utils::utils;
    use std::fs;
    use std::fs::read_to_string;
    use std::path::Path;

    pub fn load_resource(file_path: String) -> Result<String, String> {
        let complete_resource_path = format!("resources/html{}", file_path);
        let valid_resource_path = check_resource_path(&complete_resource_path);
        let content_type = ContentType::get_content_type_from_file_path(file_path);
        return match content_type {
            ContentType::HTML => Ok(utils::load_html(valid_resource_path)),
            ContentType::ICO => Ok(utils::load_ico(valid_resource_path)),
            ContentType::PNG => Ok(utils::load_png(valid_resource_path)),
        };
    }

    fn check_resource_path(file_path: &str) -> String {
        let exists = Path::new(&file_path).exists();
        return match exists {
            true => file_path.to_string(),
            false => String::from("resources/html/error.html"),
        };
    }

    fn load_html(html_file_path: String) -> String {
        return read_to_string(html_file_path).expect("Unable to read file to String");
    }

    fn load_ico(ico_file_path: String) -> String {
        let icon = fs::read(ico_file_path).expect("Unable to read icon");
        return base64::encode(&*icon);
    }

    fn load_png(html_file_path: String) -> String {
        return read_to_string(html_file_path).expect("Unable to read file to String");
    }
}

mod tests {
    use super::utils::load_resource;
    #[test]
    fn load_resource_erroneous() {
        let file_path = String::from("test.html");
        let error_file_path = String::from("resources/html/error.html");
        let expected_html = load_resource(error_file_path);
        let html = load_resource(file_path);
        assert_eq!(expected_html, html)
    }

    #[test]
    fn load_resource_valid() {
        let file_path = String::from("index.html");
        let error_file_path = String::from("resources/html/error.html");
        let expected_html = load_resource(error_file_path);
        let html = load_resource(file_path);
        assert_eq!(expected_html, html)
    }
}
