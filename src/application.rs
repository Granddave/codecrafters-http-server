use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::{request, response};
use request::{HTTPMethod, HTTPRequest};
use response::HTTPResponse;

fn send_response(mut stream: TcpStream, response: HTTPResponse) {
    stream.write_all(response.to_string().as_bytes()).unwrap();
}

pub struct Application {
    pub serve_dir: String,
}

impl Application {
    fn new() -> Application {
        Application {
            serve_dir: "public".to_string(),
        }
    }

    fn handle_request(self, request: HTTPRequest) -> HTTPResponse {
        match request.method {
            HTTPMethod::Get => match request.path.as_str() {
                "/" => HTTPResponse::ok(),
                "/user-agent" => {
                    let user_agent = request.headers.get("User-Agent").unwrap();
                    HTTPResponse {
                        status_code: 200,
                        status_text: "OK".to_string(),
                        headers: HashMap::from([
                            ("Content-type".to_string(), "text/plain".to_string()),
                            ("Content-Length".to_string(), user_agent.len().to_string()),
                        ]),
                        body: user_agent.to_string(),
                    }
                }
                "/files" => {
                    let requested_file = request.path.replace("/files/", "");

                    let mut file =
                        File::open(format!("{}/{}", self.serve_dir, requested_file)).unwrap();

                    if !file.metadata().unwrap().is_file() {
                        return HTTPResponse::not_found();
                    }

                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
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
                _ if request.path.starts_with("/echo/") => {
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
                _ => HTTPResponse::not_found(),
            },
            _ => HTTPResponse {
                status_code: 501,
                status_text: "Not Implemented".to_string(),
                headers: HashMap::new(),
                body: "".to_string(),
            },
        }
    }
}

pub fn handle_connection(mut stream: TcpStream, serve_dir: String) {
    println!("accepted new connection");

    let raw_bytes = &mut [0; 512];
    stream.read_exact(raw_bytes).unwrap();
    let string = String::from_utf8_lossy(raw_bytes).to_string();
    let request = HTTPRequest::parse(&string).unwrap();

    let app = Application { serve_dir };

    send_response(stream, app.handle_request(request));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_handle_request_simple_get() {
        use super::*;
        let app = Application::new();
        let request = HTTPRequest {
            method: HTTPMethod::Get,
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = app.handle_request(request);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.body, "");
    }

    #[test]
    fn test_echo() {
        use super::*;
        let app = Application::new();
        let request = HTTPRequest {
            method: HTTPMethod::Get,
            path: "/echo/Hello".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = app.handle_request(request);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.body, "Hello");
    }

    #[test]
    fn test_user_agent() {
        use super::*;
        let app = Application::new();
        let request = HTTPRequest {
            method: HTTPMethod::Get,
            path: "/user-agent".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::from([("User-Agent".to_string(), "Test".to_string())]),
        };
        let response = app.handle_request(request);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.body, "Test");
    }

    #[test]
    fn test_not_implemented() {
        use super::*;
        let app = Application::new();
        let request = HTTPRequest {
            method: HTTPMethod::Post,
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = app.handle_request(request);
        assert_eq!(response.status_code, 501);
        assert_eq!(response.status_text, "Not Implemented");
        assert_eq!(response.body, "");
    }
}
