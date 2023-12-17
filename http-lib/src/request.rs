use crate::http::{HttpError, HttpMethod, HttpStatusCode};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl std::str::FromStr for Request {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut raw_request = s.split("\r\n").take_while(|line| !line.is_empty());

        let mut raw_req_header = raw_request
            .next()
            .ok_or(HttpError(
                HttpStatusCode::BadRequest,
                "Malformed Request".to_owned(),
            ))?
            .split(" ");

        let mut headers: HashMap<String, String> = HashMap::new();

        raw_request.for_each(|header| {
            let mut current = header.split(": ");
            let key = current.next().unwrap().to_owned();
            let val = current.next().unwrap().to_owned();
            headers.insert(key, val);
        });

        Ok(Request {
            method: raw_req_header.next().unwrap().parse().unwrap(),
            uri: raw_req_header.next().unwrap().to_owned(),
            version: raw_req_header.next().unwrap().to_owned(),
            headers: headers,
            body: None,
        })
    }
}
