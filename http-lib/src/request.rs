use crate::http::{HttpError, HttpMethod, HttpStatusCode};
use std::{collections::HashMap, os::linux::raw};

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
        println!("Raw request: {:#?}", s);

        let lines = s
            .split("\r\n")
            .map(|line| line.to_owned())
            .collect::<Vec<String>>();

        println!("Untyped request: {:#?}", lines);

        let raw_req_header = lines[0].split(" ").collect::<Vec<&str>>();

        if raw_req_header.len() < 3 {
            return Err(HttpError(
                HttpStatusCode::BadRequest,
                "Malformed request".to_owned(),
            ));
        }

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body: Option<String> = None;
        let mut collect = false;

        for line in lines.iter().skip(1) {
            if line.is_empty() {
                collect = true;
            } else if collect {
                body = Some(line.trim_matches(char::from(0)).to_owned());
            } else {
                let current = line.split(": ").collect::<Vec<&str>>();

                if current.len() < 2 {
                    continue;
                }

                let key = current[0].to_owned();
                let val = current[1].to_owned();
                headers.insert(key, val);
            }
        }

        Ok(Request {
            method: raw_req_header[0].parse().unwrap(),
            uri: raw_req_header[1].to_owned(),
            version: raw_req_header[2].to_owned(),
            headers,
            body: if body.as_deref().unwrap_or_default().is_empty() {
                None
            } else {
                body
            },
        })
    }
}
