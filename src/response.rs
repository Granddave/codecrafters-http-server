use std::collections::HashMap;

use itertools::Itertools;

pub struct HTTPResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HTTPResponse {
    pub fn to_string(&self) -> String {
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

    pub fn ok() -> HTTPResponse {
        HTTPResponse {
            status_code: 200,
            status_text: "OK".to_string(),
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
}
