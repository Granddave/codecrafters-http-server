use std::io::{Read, Write};
use std::net::TcpListener;

use itertools::Itertools;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let request = &mut [0; 512];
                stream.read(request).unwrap();
                let text = String::from_utf8_lossy(request).to_string();
                let lines = text.lines().collect_vec();
                let start_line = lines[0].split_whitespace().collect_vec();
                if start_line[0] == "GET" {
                    match start_line[1] {
                        "/" => {
                            let response = "HTTP/1.1 200 OK\r\n\r\n";
                            stream.write(response.as_bytes()).unwrap();
                        }
                        _ if start_line[1].starts_with("/echo/") => {
                            let echo = start_line[1].replace("/echo/", "");
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
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
