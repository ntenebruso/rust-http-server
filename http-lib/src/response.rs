use crate::http::HttpStatusCode;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

#[derive(Debug, Default)]
pub struct ResponseBuilder {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl ResponseBuilder {
    pub fn new(status_code: HttpStatusCode) -> Self {
        Self {
            status_code: status_code as u16,
            status_text: status_code.default_reason_phrase(),
            headers: None,
            body: None,
        }
    }

    pub fn insert_header(&mut self, header_name: &str, header_val: &str) -> &mut Self {
        if self.headers.is_none() {
            self.headers = Some(HashMap::new());
        }

        self.headers
            .as_mut()
            .unwrap()
            .insert(header_name.to_owned(), header_val.to_owned());

        self
    }

    pub fn content_type(&mut self, new_type: &str) -> &mut Self {
        self.insert_header("Content-Type", new_type);

        self
    }

    pub fn body(&mut self, message: &str) -> Response {
        self.body = Some(message.to_owned());

        Response {
            status_code: self.status_code.clone(),
            status_text: self.status_text.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
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

        write!(
            f,
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code,
            self.status_text,
            headers,
            self.body.as_deref().unwrap_or_default()
        )
    }
}
