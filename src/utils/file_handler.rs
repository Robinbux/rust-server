const ERROR_FILE_PATH: &str = "resources/html/404.html";

pub mod file_handler {
    use crate::enums::content_type::ContentType;
    use image::ImageFormat;
    use std::fs::read_to_string;
    use std::path::Path;
    use std::str;

    pub fn load_resource(file_name: &str) -> Result<Vec<u8>, String> {
        let raw_file_name = extract_raw_file_name(file_name);
        let content_type = ContentType::get_content_type_from_file_path(&raw_file_name);
        match content_type {
            ContentType::HTML => Ok(load_text(format!("resources/html/{}", &raw_file_name))),
            ContentType::ICO => Ok(load_ico(format!("resources/assets/{}", &raw_file_name))),
            ContentType::PNG => Ok(load_png(format!("resources/assets/{}", &raw_file_name))),
            ContentType::JAVASCRIPT => Ok(load_text(format!("resources/js/{}", &raw_file_name))),
            ContentType::CSS => Ok(load_text(format!("resources/css/{}", &raw_file_name))),
            _ => panic!("Unsupported content type!"),
        }
    }

    // If the provided file_name is a path eg. /css/style.css, we cant to extract the raw file name.
    fn extract_raw_file_name(file_name: &str) -> String {
        if !file_name.contains("/") {
            return file_name.to_owned()
        }
        file_name.split("/").last().unwrap().to_owned()
    }

    fn check_resource_path(file_path: &str) -> String {
        let exists = Path::new(&file_path).exists();
        return match exists {
            true => file_path.to_string(),
            false => String::from(super::ERROR_FILE_PATH),
        };
    }

    pub fn convert_vec_to_string(vec: &Vec<u8>) -> &str {
        str::from_utf8(&vec).unwrap()
    }

    fn load_text(html_file_path: String) -> Vec<u8> {
        let valid_resource_path = check_resource_path(&html_file_path);
        read_to_string(valid_resource_path).unwrap().into_bytes()
    }

    fn load_ico(ico_file_path: String) -> Vec<u8> {
        let valid_resource_path = check_resource_path(&ico_file_path);
        let bytes: Vec<u8> = std::fs::read(valid_resource_path).unwrap();
        match image::load_from_memory_with_format(&bytes, ImageFormat::Ico) {
            Ok(_img) => {
                println!("input in ico");
            }
            Err(_) => {
                println!("input is not ico");
            }
        }
        bytes
    }

    fn load_png(png_file_path: String) -> Vec<u8> {
        let valid_resource_path = check_resource_path(&png_file_path);
        let bytes: Vec<u8> = std::fs::read(valid_resource_path).unwrap();
        match image::load_from_memory_with_format(&bytes, ImageFormat::Png) {
            Ok(_img) => {
                println!("input in png");
            }
            Err(_) => {
                println!("input is not png");
            }
        }
        bytes
    }
}

mod tests {
    #[cfg(test)]
    use super::file_handler::load_resource;
    #[cfg(test)]
    use std::fs::read_to_string;

    #[test]
    fn load_resource_erroneous() {
        let file_path = String::from("test.html");
        let error_file_path = super::ERROR_FILE_PATH;
        let expected_html = read_to_string(error_file_path.to_string()).unwrap();
        let html = load_resource(&file_path).unwrap();
        assert_eq!(expected_html, html)
    }

    #[test]
    fn load_resource_valid() {
        let file_path = String::from("index.html");
        let expected_html = read_to_string("resources/html/index.html".to_string()).unwrap();
        let html = load_resource(&file_path).unwrap();
        assert_eq!(expected_html, html)
    }
}
