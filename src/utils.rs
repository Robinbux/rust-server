pub mod utils {
    use crate::enums::content_type::ContentType;
    use std::fs::read_to_string;
    use std::fs;
    use crate::utils::utils;
    use std::path::Path;

    pub fn load_resource(file_path: String) -> Result<String, String> {
        let complete_resource_path = format!("resources{}", file_path);
        let valid_resource_path = check_resource_path(&complete_resource_path);
        let content_type = ContentType::get_content_type_from_file_path(file_path);
        return match content_type {
            ContentType::HTML => Ok(utils::load_html(valid_resource_path)),
            ContentType::ICO => Ok(utils::load_ico(valid_resource_path)),
        }
    }

    fn check_resource_path(file_path: &str) -> String{
        let exists = Path::new(&file_path).exists();
        return match exists {
            true => file_path.to_string(),
            false => String::from("resources/error.html")
        }
    }

    fn load_html(html_file_path: String) -> String {
        return read_to_string(html_file_path).expect("Unable to read file to String");
    }

    fn load_ico(ico_file_path: String) -> String {
        let icon = fs::read(ico_file_path).expect("Unable to read icon");
        return base64::encode(&*icon);
    }

    #[test]
    fn check_erroneous_resource_path(){
        let file_path = String::from("index.html");
        let error_file_path = String::from("resources/error.html");
        let result = check_resource_path(&file_path);
        assert_eq!(result, error_file_path)
    }

    #[test]
    fn check_valid_resource_path(){
        let file_path = String::from("resources/index.html");
        let result = check_resource_path(&file_path);
        assert_eq!(result, file_path)
    }

}




