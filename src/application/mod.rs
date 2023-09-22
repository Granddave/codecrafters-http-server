use std::io::{Read, Write};
use std::net::TcpStream;

use crate::{request, response};
use request::{HTTPMethod, HTTPRequest};
use response::HTTPResponse;

use self::handlers::{handle_file_request, FileReader};

mod handlers;

fn send_response(mut stream: TcpStream, response: HTTPResponse) {
    stream
        .write_all(response.to_string().as_bytes())
        .expect("Failed to write to stream");
}

pub struct Application {
    pub serve_dir: String,
}

impl Application {
    fn _new() -> Application {
        Application {
            serve_dir: "public".to_string(),
        }
    }

    fn handle_request(self, request: HTTPRequest) -> HTTPResponse {
        match request.method {
            HTTPMethod::Get => match request.path.as_str() {
                "/" => HTTPResponse::ok(),
                "/user-agent" => handlers::handle_user_agent_request(&request),
                _ if request.path.starts_with("/files/") => {
                    handle_file_request(&request, &self.serve_dir, FileReader)
                }
                _ if request.path.starts_with("/echo/") => handlers::handle_echo_request(&request),
                _ => HTTPResponse::not_found(),
            },
            _ => HTTPResponse::not_implemented(),
        }
    }
}

#[allow(clippy::unused_io_amount)]
pub fn handle_connection(mut stream: TcpStream, serve_dir: String) {
    println!("accepted new connection");

    let raw_bytes = &mut [0; 512];
    stream.read(raw_bytes).unwrap();
    let string = String::from_utf8_lossy(raw_bytes).to_string();
    let request = HTTPRequest::parse(&string).unwrap();

    let app = Application { serve_dir };
    send_response(stream, app.handle_request(request));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_handle_request_simple_get() {
        let app = Application::_new();
        let request = HTTPRequest {
            method: HTTPMethod::Get,
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = app.handle_request(request);
        assert_eq!(response, HTTPResponse::ok());
    }

    #[test]
    fn test_echo() {
        let app = Application::_new();
        let request = HTTPRequest {
            method: HTTPMethod::Get,
            path: "/echo/Hello".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = app.handle_request(request);
        assert_eq!(response, HTTPResponse::ok());
    }

    #[test]
    fn test_user_agent() {
        let app = Application::_new();
        let request = HTTPRequest {
            method: HTTPMethod::Get,
            path: "/user-agent".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::from([("User-Agent".to_string(), "Test".to_string())]),
        };
        let response = app.handle_request(request);

        let mut expected_response = HTTPResponse::ok();
        expected_response.body = "Test".to_string();
        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_not_implemented() {
        let app = Application::_new();
        let request = HTTPRequest {
            method: HTTPMethod::Post,
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = app.handle_request(request);
        assert_eq!(response, HTTPResponse::not_implemented());
    }
}
