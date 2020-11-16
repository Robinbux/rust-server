#[macro_use]
extern crate diesel;

mod controller;
mod dtos;
mod enums;
mod server;
mod services;
mod utils;
mod database;
pub mod schema;

use crate::server::server::Server;

fn main() {
    let mut server = Server::new();
    server.listen(10)
}
