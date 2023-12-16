use std::collections::HashMap;
use crate::http::HttpMethod;

#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
