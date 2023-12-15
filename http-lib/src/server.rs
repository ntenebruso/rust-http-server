use std::fs;
use std::collections::HashMap;
use std::net::TcpListener;
use std::io::prelude::*;
use std::io::BufReader;

use crate::http::HttpMethod;
use crate::request::Request;
use crate::response::Response;

pub struct Route {
    method: HttpMethod,
    path: String,
}

pub type RouteHandler = fn(Request) -> Response;

pub struct Server {
    address: Option<String>,
    routes: Option<HashMap<Route, RouteHandler>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            address: None,
            routes: Some(HashMap::new()),
        }
    }

    pub fn bind(&mut self, addr: &str) {
      self.address = Some(addr.to_owned());
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.address.as_deref().unwrap()).unwrap();

        for tcp_stream in listener.incoming() {
            let mut buffer = [0; 1024];
            let mut stream = tcp_stream.unwrap();
            stream.read(&mut buffer).unwrap();
        
            let request_str = String::from_utf8_lossy(&buffer);
            let mut raw_request = request_str.split("\r\n").take_while(|x| !x.is_empty());

            let mut raw_req_header = raw_request.next().unwrap().split(" ");

            let mut headers: HashMap<String, String> = HashMap::new();

            raw_request.for_each(|header| {
                let mut current = header.split(": ");
                let key = current.next().unwrap_or("None").to_owned();
                let val = current.next().unwrap_or("None").to_owned();
                headers.insert(key, val);
            });

            let request = Request {
                method: raw_req_header.next().unwrap().to_owned(),
                uri: raw_req_header.next().unwrap().to_owned(),
                version: raw_req_header.next().unwrap().to_owned(),
                headers: headers,
                body: None
            };

            println!("request: {:#?}", request);

            println!("connection established");

            stream.flush().unwrap();
        }
    }
}
