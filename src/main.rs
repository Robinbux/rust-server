mod mime_response;
mod server;

use crate::mime_response::MimeResponse;
use libc::in_addr;
use libc::INADDR_ANY;
use nix::sys::socket::*;
use nix::unistd::close;
use std::fs;
use std::fs::{read_to_string};
use crate::server::Server;

const PORT: u16 = 8080;

fn main() {
    let server = Server::new();
    server.listen(10)
}
