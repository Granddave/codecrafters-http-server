use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use itertools::Itertools;

fn handle(mut stream: TcpStream) {
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
            "/user-agent" => {
                let headers = lines[1..].iter().map(|x| x.to_string()).collect_vec();
                let user_agent = headers
                    .iter()
                    .find(|x| x.starts_with("User-Agent: "))
                    .unwrap()
                    .replace("User-Agent: ", "")
                    .to_string();
                let res = format!(
                    "Content-type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent.len(),
                    user_agent,
                );
                let response = format!("HTTP/1.1 200 OK\r\n{}", res);
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
