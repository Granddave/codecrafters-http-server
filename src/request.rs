use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Debug)]
pub enum HTTPMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(PartialEq, Debug)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

fn parse_headers(headers: Vec<&str>) -> HashMap<String, String> {
    let mut header_map = HashMap::new();

    for line in headers {
        if let Some((key, value)) = parse_header_line(line) {
            header_map.insert(key, value);
        }
    }

    header_map
}

fn parse_header_line(line: &str) -> Option<(String, String)> {
    let mut parts = line.splitn(2, ':');
    if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
        let key = key.trim().to_string();
        let value = value.trim().to_string();
        Some((key, value))
    } else {
        None
    }
}

impl HTTPRequest {
    pub fn parse(request: &str) -> Option<HTTPRequest> {
        let lines = request.lines().collect_vec();
        let start_line = lines[0].split_whitespace().collect_vec();
        Some(HTTPRequest {
            method: match start_line[0] {
                "GET" => HTTPMethod::Get,
                "POST" => HTTPMethod::Post,
                "PUT" => HTTPMethod::Put,
                "DELETE" => HTTPMethod::Delete,
                _ => return None,
            },
            path: start_line[1].to_string(),
            version: start_line[2].to_string(),
            headers: parse_headers(lines[1..].to_vec()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request() {
        let request = HTTPRequest::parse(
            "GET / HTTP/1.1\r\n\
             Host: localhost:4221\r\n\
             User-Agent: curl/7.54.0\r\n\
             Accept: */*\r\n\r\n",
        )
        .unwrap();
        assert_eq!(request.method, HTTPMethod::Get);
        assert_eq!(request.path, "/");
        assert_eq!(request.version, "HTTP/1.1");
        assert_eq!(request.headers.len(), 3);
        assert_eq!(
            request.headers.get("Host"),
            Some(&"localhost:4221".to_string())
        );
        assert_eq!(
            request.headers.get("User-Agent"),
            Some(&"curl/7.54.0".to_string())
        );
        assert_eq!(request.headers.get("Accept"), Some(&"*/*".to_string()));
    }

    #[test]
    fn test_parse_header_line() {
        assert_eq!(parse_header_line(""), None);
        assert_eq!(parse_header_line("invalid"), None);
        assert_eq!(
            parse_header_line("Host: localhost:4221"),
            Some(("Host".to_string(), "localhost:4221".to_string()))
        );
        assert_eq!(
            parse_header_line("User-Agent: curl/7.54.0"),
            Some(("User-Agent".to_string(), "curl/7.54.0".to_string()))
        );
        assert_eq!(
            parse_header_line("Accept: */*"),
            Some(("Accept".to_string(), "*/*".to_string()))
        );
    }

    #[test]
    fn test_parse_headers() {
        let headers = parse_headers(vec![
            "Host: localhost:4221",
            "User-Agent: curl/7.54.0",
            "Accept: */*",
        ]);
        assert_eq!(headers.len(), 3);
        assert_eq!(headers.get("Host"), Some(&"localhost:4221".to_string()));
        assert_eq!(headers.get("User-Agent"), Some(&"curl/7.54.0".to_string()));
        assert_eq!(headers.get("Accept"), Some(&"*/*".to_string()));
    }
}
