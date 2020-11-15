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

// fn main() {
//     let a = "This is\r\n\r\na test";
//     let mut a_vec = a.as_bytes().to_owned();
//
//     let index = find_payload_index(&a_vec);
//     let splitted_buf = a_vec.split_at(index).1.to_vec();
//     let splittted_str = String::from_utf8(splitted_buf).unwrap();
//
//     let test = 5;
// }
//
// fn find_payload_index(buffer: &Vec<u8>) -> usize {
//     for (pos, e) in buffer.iter().enumerate() {
//         if pos < 3 {
//             continue
//         }
//         if buffer[pos - 3] == 13 && buffer[pos - 2] == 10 && buffer[pos - 1] == 13 && buffer[pos] == 10 {
//             return pos + 1;
//         }
//     }
//     0
// }