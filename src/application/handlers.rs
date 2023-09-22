use std::{collections::HashMap, fs::File, io::Read};

use crate::{request::HTTPRequest, response::HTTPResponse};

pub trait FileReading {
    fn read_to_string(&mut self, path: &str) -> Result<String, std::io::Error>;
}

pub struct FileReader;
impl FileReading for FileReader {
    fn read_to_string(&mut self, path: &str) -> Result<String, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

pub fn handle_file_request(
    request: &HTTPRequest,
    serve_dir: &str,
    mut file_reader: impl FileReading,
) -> HTTPResponse {
    let requested_file = request.path.replace("/files/", "");
    if requested_file.is_empty() {
        return HTTPResponse::bad_request();
    }

    let contents =
        match file_reader.read_to_string(format!("{}/{}", serve_dir, requested_file).as_str()) {
            Ok(contents) => contents,
            Err(_) => return HTTPResponse::not_found(),
        };

    HTTPResponse {
        status_code: 200,
        status_text: "OK".to_string(),
        headers: HashMap::from([
            (
                "Content-type".to_string(),
                "application/octet-stream".to_string(),
            ),
            ("Content-Length".to_string(), contents.len().to_string()),
        ]),
        body: contents,
    }
}

pub fn handle_user_agent_request(request: &HTTPRequest) -> HTTPResponse {
    match request.headers.get("User-Agent") {
        Some(user_agent) => HTTPResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::from([
                ("Content-type".to_string(), "text/plain".to_string()),
                ("Content-Length".to_string(), user_agent.len().to_string()),
            ]),
            body: user_agent.to_string(),
        },
        None => HTTPResponse::bad_request(),
    }
}

pub fn handle_echo_request(request: &HTTPRequest) -> HTTPResponse {
    let echo = request.path.replace("/echo/", "");
    HTTPResponse {
        status_code: 200,
        status_text: "OK".to_string(),
        headers: HashMap::from([
            ("Content-type".to_string(), "text/plain".to_string()),
            ("Content-Length".to_string(), echo.len().to_string()),
        ]),
        body: echo.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::HTTPMethod;

    struct MockFileReader {
        pub result: Result<String, std::io::Error>,
    }

    impl FileReading for MockFileReader {
        fn read_to_string(&mut self, _path: &str) -> Result<String, std::io::Error> {
            match &self.result {
                Ok(contents) => Ok(contents.to_string()),
                Err(e) => Err(std::io::Error::new(e.kind(), e.to_string())),
            }
        }
    }

    #[test]
    fn test_file_read() {
        let result = handle_file_request(
            &HTTPRequest {
                method: HTTPMethod::Get,
                path: "/files/test.txt".to_string(),
                version: "HTTP/1.1".to_string(),
                headers: HashMap::new(),
            },
            "public",
            MockFileReader {
                result: Ok("Hello".to_string()),
            },
        );
        let expected = HTTPResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::from([
                (
                    "Content-type".to_string(),
                    "application/octet-stream".to_string(),
                ),
                ("Content-Length".to_string(), "5".to_string()),
            ]),
            body: "Hello".to_string(),
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_file_read_not_found() {
        let result = handle_file_request(
            &HTTPRequest {
                method: HTTPMethod::Get,
                path: "/files/test.txt".to_string(),
                version: "HTTP/1.1".to_string(),
                headers: HashMap::new(),
            },
            "public",
            MockFileReader {
                result: Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                )),
            },
        );
        let expected = HTTPResponse {
            status_code: 404,
            status_text: "Not Found".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
        };
        assert_eq!(result, expected);
    }
}
