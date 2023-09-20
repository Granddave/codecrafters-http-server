use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use itertools::Itertools;

#[derive(PartialEq)]
enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

struct HTTPRequest {
    method: HTTPMethod,
    path: String,
    version: String,
    headers: HashMap<String, String>,
}

impl HTTPRequest {
    fn parse(request: &str) -> Option<HTTPRequest> {
        let lines = request.lines().collect_vec();
        let start_line = lines[0].split_whitespace().collect_vec();
        Some(HTTPRequest {
            method: match start_line[0] {
                "GET" => HTTPMethod::GET,
                "POST" => HTTPMethod::POST,
                "PUT" => HTTPMethod::PUT,
                "DELETE" => HTTPMethod::DELETE,
                _ => None?,
            },
            path: start_line[1].to_string(),
            version: start_line[2].to_string(),
            headers: lines[1..]
                .iter()
                .map(|x| {
                    let mut parts = x.splitn(2, ": ");
                    (
                        parts.next().unwrap().to_string(),
                        parts.next().unwrap().to_string(),
                    )
                })
                .collect(),
        })
    }
}

struct HTTPResponse {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HTTPResponse {
    fn to_string(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .join("\r\n");
        format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code, self.status_text, headers, self.body
        )
    }

    fn ok() -> HTTPResponse {
        HTTPResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
        }
    }
}

fn send_response(mut stream: TcpStream, response: HTTPResponse) {
    stream.write(response.to_string().as_bytes()).unwrap();
}

fn handle(mut stream: TcpStream) {
    println!("accepted new connection");

    let request = &mut [0; 512];
    stream.read(request).unwrap();
    let text = String::from_utf8_lossy(request).to_string();
    let request = HTTPRequest::parse(&text).unwrap();
    if request.method == HTTPMethod::GET {
        match request.path.as_str() {
            "/" => send_response(stream, HTTPResponse::ok()),
            "/user-agent" => {
                let user_agent = request.headers.get("User-Agent").unwrap();
                send_response(
                    stream,
                    HTTPResponse {
                        status_code: 200,
                        status_text: "OK".to_string(),
                        headers: HashMap::from([
                            ("Content-type".to_string(), "text/plain".to_string()),
                            ("Content-Length".to_string(), user_agent.len().to_string()),
                        ]),
                        body: user_agent.to_string(),
                    },
                );
            }
            _ if request.path.starts_with("/echo/") => {
                let echo = request.path.replace("/echo/", "");
                let res = format!(
                    "Content-type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    echo.len(),
                    echo,
                );
                let response = format!("HTTP/1.1 200 OK\r\n{}", res);
                stream.write(response.as_bytes()).unwrap();
            }
            _ => {
                let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
            }
        }
    }
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
