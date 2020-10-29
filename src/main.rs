mod controller;
mod enums;
mod server;
mod services;
mod utils;
mod dtos,

use crate::server::server::Server;

fn main() {
    let mut server = Server::new();
    server.listen(10)
}
