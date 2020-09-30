mod mime_response;
mod server;
mod enums;
mod utils;

use crate::server::Server;

fn main() {
    let mut server = Server::new();
    server.listen(10)
}
