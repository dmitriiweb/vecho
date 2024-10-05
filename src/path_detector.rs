/// Path parser, will detect what kind of path it is: "local" or "url"
pub fn path_detector(path: &str) -> String {
    if path.starts_with("http") {
        "url".to_string()
    } else {
        "local".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_path() {
        let result = path_detector("http://example.com");
        assert_eq!(result, "url");
    }

    #[test]
    fn test_https_path() {
        let result = path_detector("https://example.com");
        assert_eq!(result, "url");
    }

    #[test]
    fn test_local_path() {
        let result = path_detector("/home/user/file.txt");
        assert_eq!(result, "local");
    }
    #[test]
    fn test_local_windows_path() {
        let result = path_detector("C:/home/user/file.txt");
        assert_eq!(result, "local");
    }

    #[test]
    fn test_relative_local_path() {
        let result = path_detector("file.txt");
        assert_eq!(result, "local");
    }
}
