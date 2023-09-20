mod application;
mod request;
mod response;

use application::handle_connection;

use std::net::TcpListener;
use std::{env, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let serve_dir = "public".to_string();

    let serve_dir = match env::args().nth(2) {
        Some(dir) => dir,
        None => serve_dir,
    };

    println!("Serving directory: {}", serve_dir);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let serve_dir = serve_dir.clone();
                thread::spawn(move || {
                    handle_connection(stream, serve_dir);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
