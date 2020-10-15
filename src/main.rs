mod controller;
mod enums;
mod server;
mod utils;

use crate::server::server::Server;

fn main() {
    let mut server = Server::new();
    server.listen(10)
}
