#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ContentType {
    HTML,
    ICO,
    PNG,
    JSON,
    JAVASCRIPT,
    CSS,
    MP4,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ContentType::HTML => "text/html",
            ContentType::ICO => "image/x-icon",
            ContentType::PNG => "image/png",
            ContentType::JSON => "application/json",
            ContentType::JAVASCRIPT => "text/javascript",
            ContentType::CSS => "text/css",
            ContentType::MP4 => "video/mp4",
        }
    }

    pub fn from_str(content_type: &str) -> Result<ContentType, String> {
        let result = match content_type.replace(";", "").as_ref() {
            "html" => Ok(ContentType::HTML),
            "ico" => Ok(ContentType::ICO),
            "png" => Ok(ContentType::PNG),
            "application/json" => Ok(ContentType::JSON), // TODO: Check if correct,
            "js" => Ok(ContentType::JAVASCRIPT),
            "css" => Ok(ContentType::CSS),
            "mp4" => Ok(ContentType::MP4),
            _ => Err(String::from("Unable to convert given String to ContentType")),
        };
        result
    }

    pub fn get_content_type_from_file_path(path: &str) -> Result<ContentType, String> {
        let content_type_str = path.split('.').last().unwrap();
        let result = ContentType::from_str(content_type_str);

        if result.is_err(){
            return Err(result.unwrap_err());
        };
        Ok(result.unwrap())
    }
}
#[cfg(test)]
mod tests {
    use super::ContentType;

    #[test]
    fn from_str_html() {
        let file_path = "html;";
        let result = ContentType::from_str(file_path);
        assert_eq!(result.unwrap(), ContentType::HTML)
    }

    #[test]
    fn from_str_ico() {
        let file_path = "ico";
        let result = ContentType::from_str(file_path);
        assert_eq!(result.unwrap(), ContentType::ICO)
    }

    #[test]
    fn from_str_false() {
        let file_path = "index";
        let result = ContentType::from_str(file_path);
        assert!(result.is_err())
    }

    #[test]
    fn get_content_type_from_file_path_missing() {
        let file_path = "/home/portfolio";
        let result = ContentType::get_content_type_from_file_path(file_path);
        let error_msg = String::from("Unable to convert given String to ContentType");
        assert_eq!(result.unwrap_err(), error_msg)
    }

    #[test]
    fn get_content_type_from_file_path_html() {
        let file_path = String::from("/home/portfolio.html");
        let result = ContentType::get_content_type_from_file_path(&file_path).unwrap();
        assert_eq!(result, ContentType::HTML)
    }

    #[test]
    fn get_content_type_from_file_path_ico() {
        let file_path = String::from("/fav.ico");
        let result = ContentType::get_content_type_from_file_path(&file_path).unwrap();
        assert_eq!(result, ContentType::ICO)
    }
}
