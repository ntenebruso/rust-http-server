use std::collections::HashMap;

pub struct Response {
    pub version: String,
    pub status_code: String,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
