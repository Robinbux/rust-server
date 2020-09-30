mod mime_response;
mod server;
mod content_type;

use crate::server::Server;

const PORT: u16 = 8080;

fn main() {
    let mut server = Server::new();
    server.listen(10)
}
