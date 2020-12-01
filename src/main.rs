#[macro_use]
extern crate diesel;

mod controller;
mod database;
mod dtos;
mod enums;
pub mod schema;
mod server;
mod services;
mod thread_pool;
mod utils;

use crate::server::server::Server;

fn main() {
    let server = Server::new();
    server.listen(10)
}
