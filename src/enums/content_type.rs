#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ContentType {
    HTML,
    ICO,
    PNG,
    JSON,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ContentType::HTML => "text/html",
            ContentType::ICO => "image/x-icon",
            ContentType::PNG => "image/png",
            ContentType::JSON => "application/json",
        }
    }

    pub fn from_str(content_type: String) -> Result<ContentType, ()> {
        let result = match content_type.replace(";", "").as_ref() {
            "html" => Ok(ContentType::HTML),
            "ico" => Ok(ContentType::ICO),
            "png" => Ok(ContentType::PNG),
            "application/json" => Ok(ContentType::JSON), // TODO: Check if correct
            _ => Err(()),                                // TODO: Error handling!
        };
        return result;
    }

    pub fn get_content_type_from_file_path(path: &str) -> ContentType {
        let content_type_str = path.split(".").last().expect("Unable to split path.");
        let result = ContentType::from_str(String::from(content_type_str));

        let unwrapped = result.expect("Unable to convert given String to ContentType.");
        return unwrapped;
    }
}
#[cfg(test)]
mod tests {
    use super::ContentType;

    #[test]
    fn from_str_html() {
        let file_path = String::from("html");
        let result = ContentType::from_str(file_path);
        assert_eq!(result.unwrap(), ContentType::HTML)
    }

    #[test]
    fn from_str_ico() {
        let file_path = String::from("ico");
        let result = ContentType::from_str(file_path);
        assert_eq!(result.unwrap(), ContentType::ICO)
    }

    #[test]
    fn from_str_false() {
        let file_path = String::from("index");
        let result = ContentType::from_str(file_path);
        assert_eq!(result, Err(()))
    }

    #[test]
    #[should_panic]
    fn get_content_type_from_file_path_missing() {
        let file_path = String::from("/home/portfolio");
        ContentType::get_content_type_from_file_path(&file_path);
    }

    #[test]
    fn get_content_type_from_file_path_html() {
        let file_path = String::from("/home/portfolio.html");
        let result = ContentType::get_content_type_from_file_path(&file_path);
        assert_eq!(result, ContentType::HTML)
    }

    #[test]
    fn get_content_type_from_file_path_ico() {
        let file_path = String::from("/fav.ico");
        let result = ContentType::get_content_type_from_file_path(&file_path);
        assert_eq!(result, ContentType::ICO)
    }
}
