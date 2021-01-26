use rust_server::net::server::Server;

fn main() {
    let server = Server::new();
    server.listen(10)
}
