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
        let listener = TcpListener::bind(&self.address.as_deref().unwrap()).unwrap();

        for stream in listener.incoming() {
            let mut buffer = [0; 1024];
            let _ = stream.unwrap().read(&mut buffer);

            println!("Buffer: {:?}", buffer);

            println!("connection established");
        }
    }
}
