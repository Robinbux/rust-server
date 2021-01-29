use rust_server::Server;

fn main() {
    let server = Server::new();
    server.listen(10)
}
