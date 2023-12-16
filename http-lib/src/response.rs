use crate::http::HttpStatusCode;
use std::collections::HashMap;
use std::fmt;

pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl Response {
    pub fn send_text(status_code: HttpStatusCode, body: &str) -> Self {
        Self {
            status_code: status_code as u16,
            status_text: status_code.default_reason_phrase(),
            headers: None,
            body: Some(body.to_owned())
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let headers = if self.headers.is_some() {
            self.headers
            .as_ref()
            .unwrap()
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n")
        } else {
            String::default()
        };

        write!(f, "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code,
            self.status_text,
            headers,
            self.body.as_deref().unwrap_or_default()
        )
    }
}
