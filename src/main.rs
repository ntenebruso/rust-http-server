use http_lib::http::HttpMethod;
use http_lib::http::HttpStatusCode;
use http_lib::response::ResponseBuilder;
use http_lib::server::Server;
use std::fs;

fn main() {
    let mut server = Server::new();
    server.bind("0.0.0.0:3000");

    server.route(HttpMethod::Get, "/", |req| {
        ResponseBuilder::new(HttpStatusCode::Success)
            .content_type("text/html")
            .body(fs::read_to_string("html/index.html").unwrap().as_str())
    });

    server.route(HttpMethod::Get, "/about", |req| {
        ResponseBuilder::new(HttpStatusCode::Success).body("about")
    });

    server.run();
}
