use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;

use crate::http::{HttpError, HttpMethod, HttpStatusCode};
use crate::request::Request;
use crate::response::Response;

pub type RouteHandler = fn(Request) -> Response;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Route {
    method: HttpMethod,
    path: String,
}

pub struct ServerBuilder {
    address: Option<String>,
    routes: HashMap<Route, RouteHandler>,
}

impl ServerBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            routes: HashMap::new(),
        }
    }

    pub fn bind(&mut self, addr: &str) -> &mut Self {
        self.address = Some(addr.to_owned());

        self
    }

    pub fn route(&mut self, method: HttpMethod, route: &str, handler: RouteHandler) -> &mut Self {
        self.routes.insert(
            Route {
                method: method,
                path: route.to_owned(),
            },
            handler,
        );

        self
    }

    pub fn build(&mut self) -> Server {
        Server {
            address: self.address.clone().unwrap_or("0.0.0.0:3000".to_owned()),
            routes: self.routes.clone(),
        }
    }
}

pub struct Server {
    address: String,
    routes: HashMap<Route, RouteHandler>,
}

impl Server {
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();

        for tcp_stream in listener.incoming() {
            let mut buffer = [0; 1024];
            let mut stream = tcp_stream.unwrap();
            stream.read(&mut buffer).unwrap();

            let request_str = String::from_utf8_lossy(&buffer);
            let request: Request = request_str.parse().unwrap();

            println!("request: {:#?}", request);

            let response = handle_route(request, &self.routes);

            match response {
                Ok(res) => {
                    let response_string = res.to_string();
                    stream.write(response_string.as_bytes()).unwrap();
                }
                Err(e) => {
                    let error_response = Response {
                        status_code: e.0 as u16,
                        status_text: e.0.default_reason_phrase(),
                        headers: None,
                        body: Some(e.1.to_owned()),
                    };

                    stream.write(error_response.to_string().as_bytes()).unwrap();
                }
            }

            stream.flush().unwrap();
        }
    }
}

fn handle_route(
    request: Request,
    routes: &HashMap<Route, RouteHandler>,
) -> Result<Response, HttpError> {
    if let Some(handler) = routes.get(&Route {
        method: request.method.clone(),
        path: request.uri.clone(),
    }) {
        Ok(handler(request))
    } else {
        Err(HttpError(
            HttpStatusCode::NotFound,
            format!("Cannot GET {}", request.uri),
        ))
    }
}
