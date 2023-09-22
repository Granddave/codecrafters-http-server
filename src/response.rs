use core::fmt;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Debug)]
pub struct HTTPResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HTTPResponse {
    pub fn ok() -> HTTPResponse {
        HTTPResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
        }
    }

    pub fn bad_request() -> HTTPResponse {
        HTTPResponse {
            status_code: 400,
            status_text: "Bad Request".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
        }
    }

    pub fn not_found() -> HTTPResponse {
        HTTPResponse {
            status_code: 404,
            status_text: "Not Found".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
        }
    }
    pub fn not_implemented() -> HTTPResponse {
        HTTPResponse {
            status_code: 501,
            status_text: "Not Implemented".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
        }
    }
}

impl fmt::Display for HTTPResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .join("\r\n");
        write!(
            f,
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code, self.status_text, headers, self.body
        )
    }
}
