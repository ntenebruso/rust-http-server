use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
