use http_lib::response::Response;
use http_lib::http::HttpStatusCode;
use http_lib::server::Server;
use http_lib::http::HttpMethod;

fn main() {
    let mut server = Server::new();
    server.bind("0.0.0.0:3000");

    server.route(HttpMethod::Get, "/", |req| {
        Response::send_text(HttpStatusCode::Success, "test")
    });

    server.route(HttpMethod::Get, "/about", |req| {
        Response::send_text(HttpStatusCode::Success, "about page")
    });

    server.run();
}
