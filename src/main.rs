mod request;
mod response;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use request::{HTTPMethod, HTTPRequest};
use response::HTTPResponse;

fn send_response(mut stream: TcpStream, response: HTTPResponse) {
    stream.write(response.to_string().as_bytes()).unwrap();
}

fn handle_request(request: HTTPRequest) -> HTTPResponse {
    match request.method {
        HTTPMethod::GET => match request.path.as_str() {
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_handle_request_simple_get() {
        use super::*;
        let request = HTTPRequest {
            method: HTTPMethod::GET,
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = handle_request(request);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.body, "");
    }

    #[test]
    fn test_echo() {
        use super::*;
        let request = HTTPRequest {
            method: HTTPMethod::GET,
            path: "/echo/Hello".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = handle_request(request);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.body, "Hello");
    }

    #[test]
    fn test_user_agent() {
        use super::*;
        let request = HTTPRequest {
            method: HTTPMethod::GET,
            path: "/user-agent".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::from([("User-Agent".to_string(), "Test".to_string())]),
        };
        let response = handle_request(request);
        assert_eq!(response.status_code, 200);
        assert_eq!(response.status_text, "OK");
        assert_eq!(response.body, "Test");
        assert_eq!(
            response.headers.get("Content-Length"),
            Some(&"4".to_string())
        );
    }

    #[test]
    fn test_not_found() {
        use super::*;
        let request = HTTPRequest {
            method: HTTPMethod::GET,
            path: "/not-found".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = handle_request(request);
        assert_eq!(response.status_code, 404);
        assert_eq!(response.status_text, "Not Found");
        assert_eq!(response.body, "");
    }

    #[test]
    fn test_not_implemented() {
        use super::*;
        let request = HTTPRequest {
            method: HTTPMethod::POST,
            path: "/".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
        };
        let response = handle_request(request);
        assert_eq!(response.status_code, 501);
        assert_eq!(response.status_text, "Not Implemented");
        assert_eq!(response.body, "");
    }
}

fn handle(mut stream: TcpStream) {
    println!("accepted new connection");

    let raw_bytes = &mut [0; 512];
    stream.read(raw_bytes).unwrap();
    let string = String::from_utf8_lossy(raw_bytes).to_string();
    let request = HTTPRequest::parse(&string).unwrap();

    send_response(stream, handle_request(request));
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
