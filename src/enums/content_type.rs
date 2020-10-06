#[derive(PartialEq)]
#[derive(Debug)]
pub enum ContentType {
    HTML,
    ICO,
    PNG
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ContentType::HTML => "text/html",
            ContentType::ICO => "image/x-icon",
            ContentType::PNG => "image/png"
        }
    }

    pub fn from_str(content_type: String) -> Result<ContentType, ()> {
        let result = match content_type.as_ref() {
            "html" => Ok(ContentType::HTML),
            "ico" => Ok(ContentType::ICO),
            "png" => Ok(ContentType::PNG),
            _ => Err(()) // TODO: Error handling!
        };
        return result;
    }

    pub fn get_content_type_from_file_path(path: String) -> ContentType {
        let content_type_str = path.split(".").last().expect("Unable to split path.");
        let result = ContentType::from_str(String::from(content_type_str));
        println!("BEFORE UNWRAPPING");
        let unwrapped = result.unwrap();
        println!("AFTER UNWRAPPING");
        println!("VALUE:\n{}", unwrapped.as_str());

        return unwrapped
    }
}

#[test]
fn test_from_str() { //
    let file_path = String::from("html");
    let result = ContentType::from_str(file_path);
    assert_eq!(result.unwrap(), ContentType::HTML)
}