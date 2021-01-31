mod services;
pub mod http;
mod utils;
mod path_handler;
pub mod enums;
mod dtos;
mod database;
pub mod controller;
mod schema;
mod thread_pool;

#[macro_use]
extern crate diesel;

use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::http::mime_response::MimeResponse;
use crate::http::request;
use crate::services::error_service::ErrorService;
use crate::thread_pool::ThreadPool;
use crate::utils::logger::Logger;
use libc::in_addr;
use libc::INADDR_ANY;
use nix::sys::socket::*;
use nix::unistd::close;
use std::os::unix::io::RawFd;
use std::sync::Arc;

const PORT: u16 = 8087;

#[derive(Clone)]
pub struct Server {
    server_fd: RawFd,
    logger: Logger,
    base_controller: BaseController,
    error_service: ErrorService,
}

impl Server {
    pub fn new() -> Server {
        let logger = Logger::new(String::from("Server"));
        let server_fd = Server::setup(&logger);
        let base_controller = BaseController::new();
        let mut error_service = ErrorService::new();

        Server {
            server_fd,
            logger,
            base_controller,
            error_service,
        }
    }

    fn setup(logger: &Logger) -> RawFd {
        let server_fd = socket(
            AddressFamily::Inet,
            SockType::Stream,
            SockFlag::empty(),
            None,
        ).expect("Unable to create socket");
        let in_address = in_addr { s_addr: INADDR_ANY };

        let sockaddr_in = nix::sys::socket::sockaddr_in {
            sin_len: 255,
            sin_family: libc::AF_INET as u8,
            sin_port: PORT.to_be(),
            sin_addr: in_address,
            sin_zero: [0; 8],
        };

        bind(server_fd, &SockAddr::Inet(InetAddr::V4(sockaddr_in))).expect("Binding failed");
        server_fd
    }

    pub fn listen(self, backlog: usize) {
        listen(self.server_fd, backlog).expect("Listening failed");
        self.logger.log("Server started listening.");

        let logical_cpus = num_cpus::get();
        let pool = ThreadPool::new(logical_cpus);
        let server = Arc::new(self);

        loop {
            println!("\n+++++++ Waiting for new connection ++++++++\n\n");
            let new_socket = accept(server.server_fd).expect("Accepting failed");
            let server = server.clone();

            pool.execute(move || {
                Server::handle_connection(server, new_socket);
            });
        }
    }

    fn handle_connection(server: Arc<Server>, connection_socket: RawFd) {
        let buffer = Server::read_into_buffer(&server, &connection_socket);
        Server::send_response_to_socket(&server, connection_socket, buffer);
        close(connection_socket).expect("Closing of Socket RawFD failed.");
    }

    fn read_into_buffer(server: &Arc<Server>, connection_socket: &RawFd) -> String {
        let mut buffer = vec![0; 30000];
        let request_str: String;

        recvfrom(*connection_socket, &mut *buffer).expect("Reading Failed");
        request_str = String::from_utf8_lossy(buffer.as_slice())
            .parse()
            .expect("Parsing Failed");

        println!("---Client Request---\n{}", request_str);
        server.logger.log("Received client request!");
        request_str
    }

    fn send_response_to_socket(server: &Arc<Server>, new_socket: RawFd, buffer: String) {
        let mime_response = Server::create_response(&server, buffer);

        send(new_socket, &mime_response.as_ref(), MsgFlags::empty()).expect("failed to send");
        server.logger.log("Response sent.");
        println!("------------------Response sent-------------------\n");
    }

    fn create_response(server: &Arc<Server>, buffer: String) -> Vec<u8> {
        let request = request::Request::new(buffer);
        let response = server.base_controller.execute_request(request);

        let mut mime_skeleton = MimeResponse {
            http_status_code: response.http_status_code,
            content_type: response.content_type,
            content_length: response.content_bytes.len(),
        };

        let mut mime_response: Vec<u8> = mime_skeleton.build_mime_response().into_bytes().to_vec();
        mime_response.extend(response.content_bytes);
        mime_response
    }
}

mod tests {
    use super::*;
    use chrono::{DateTime, Local};
    use std::fs::read_to_string;
    use std::thread;

    //use std::env;
    //use std::thread;
    use reqwest::Result;
    use reqwest::StatusCode;
    use std::time::Duration;

    fn start_server() {
        let server = Server::new();
        server.listen(10);
        thread::sleep(Duration::from_millis(100))
    }

    fn request() -> Result<reqwest::Response> {
        reqwest::get("http://localhost:8087/home")
    }

    #[test]
    fn listen() {
        thread::spawn(|| start_server());
        let response = request().unwrap();
        assert_eq!(response.status(), StatusCode::OK)
    }
}
