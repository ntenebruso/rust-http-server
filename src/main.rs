use http_lib::server::Server;

fn main() {
    let mut server = Server::new();
    server.bind("0.0.0.0:3000");
    server.run();
}
