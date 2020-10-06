pub mod utils {
    use crate::enums::content_type::ContentType;
    use std::fs::read_to_string;
    use std::fs;
    use crate::utils::utils;
    use std::path::Path;

    pub fn load_resource(file_path: String) -> Result<String, String> {
        let complete_resource_path = format!("resources{}", file_path);
        let valid_resource_path = check_resource_path(complete_resource_path);
        let content_type = ContentType::get_content_type_from_file_path(file_path);
        return match content_type {
            ContentType::HTML => Ok(utils::load_html(valid_resource_path)),
            ContentType::ICO => Ok(utils::load_ico(valid_resource_path)),
            ContentType::PNG => Ok(utils::load_png(valid_resource_path)),
        }
    }

    fn check_resource_path(file_path: String) -> String{
        let exists = Path::new(&file_path).exists();
        return match exists {
            true => file_path,
            false => String::from("resources/404.html")
        }
    }

    fn load_html(html_file_path: String) -> String {
        return read_to_string(html_file_path).expect("Unable to read file to String");
    }

    fn load_ico(ico_file_path: String) -> String {
        let icon = fs::read(ico_file_path).expect("Unable to read icon");
        return base64::encode(&*icon);
    }

    fn load_png(png_file_path: String) -> String {
        String::from_utf8_lossy(&*fs::read(png_file_path).unwrap()).parse().expect("Unable to read PNG")
    }
}