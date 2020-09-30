pub enum ContentType {
    HTML,
    ICO
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ContentType::HTML => "text/html",
            ContentType::ICO => "image/x-icon"
        }
    }

    pub fn from_str(content_type: &str) -> ContentType {
        match content_type {
            "html" => ContentType::HTML,
            "ico" => ContentType::ICO,
            _ => ContentType::HTML // TODO: Error handling!
        }
    }
}