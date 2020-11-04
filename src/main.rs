mod controller;
mod dtos;
mod enums;
mod server;
mod services;
mod utils;
use crate::server::server::Server;

fn main() {
    let mut server = Server::new();
    server.listen(10)
}
