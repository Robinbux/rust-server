mod controller;
mod dtos;
mod enums;
mod server;
mod services;
mod utils;
mod database;
use crate::server::server::Server;
use crate::database::db_handler::db_handler;

fn main() {
    db_handler::setup_database();
    let mut server = Server::new();
    server.listen(10)
}
