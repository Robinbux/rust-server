mod {
    pub fn serve_content(path: String) -> String {
        let route_beginning = extract_first_path_part(path)
        return match path {
            "admin" => 
        }
    }

    fn extract_first_path_part(path: &str) -> String {
        path.split("/")
        .nth(0)
        .expect("Unable to split result")
    }
}