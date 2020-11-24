use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::server::mime_response::MimeResponse;
use crate::server::request;
use crate::services::error_service::ErrorService;
use crate::utils::logger::Logger;
use libc::in_addr;
use libc::INADDR_ANY;
use nix::sys::socket::*;
use nix::unistd::close;
use num_cpus;
use rust_server::ThreadPool;
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
        let mut error_service = ErrorService::new();
        let server_fd = Server::setup(&mut error_service);
        let logger = Logger::new(String::from("Server"));
        let base_controller = BaseController::new();
        Server {
            server_fd,
            logger,
            base_controller,
            error_service,
        }
    }

    fn setup(error_service: &mut ErrorService) -> RawFd {
        let server_fd = socket(
            AddressFamily::Inet,
            SockType::Stream,
            SockFlag::empty(),
            None,
        );
        if server_fd.is_err() {
            error_service.serve_500_response("Unable to create socket".to_string());
        }
        let server_fd = server_fd.unwrap();
        let in_address = in_addr { s_addr: INADDR_ANY };

        let sockaddr_in = nix::sys::socket::sockaddr_in {
            sin_len: 255,
            sin_family: libc::AF_INET as u8,
            sin_port: PORT.to_be(),
            sin_addr: in_address,
            sin_zero: [0; 8],
        };

        if bind(server_fd, &SockAddr::Inet(InetAddr::V4(sockaddr_in))).is_err() {
            error_service.serve_500_response("Binding failed".to_string());
            panic!("Binding failed")
        }
        server_fd
    }

    pub fn listen(self, backlog: usize) {
        if listen(self.server_fd, backlog).is_err() {
            self.error_service
                .serve_500_response("Listening failed".to_string());
        };
        self.logger.log("Server started listening.");

        let logical_cpus = num_cpus::get();
        let pool = ThreadPool::new(logical_cpus);
        let server = Arc::new(self);

        loop {
            println!("\n+++++++ Waiting for new connection ++++++++\n\n");
            let new_socket = accept(server.server_fd);
            if new_socket.is_err() {
                server
                    .error_service
                    .serve_500_response("Accepting failed".to_string());
            };
            let new_socket = new_socket.unwrap();

            let server = server.clone();

            pool.execute(move || {
                Server::handle_connection(server, new_socket);
            });
        }
    }

    fn handle_connection(server: Arc<Server>, connection_socket: RawFd) {
        let buffer = Server::read_into_buffer(&server, &connection_socket);
        Server::send_response_to_socket(&server, connection_socket, buffer);
        if close(connection_socket).is_err() {
            server.logger.log("Closing of Socket RawFD failed.");
        }
    }

    fn read_into_buffer(server: &Arc<Server>, connection_socket: &RawFd) -> Vec<u8> {
        let mut buffer = vec![0; 30000];
        let val_read_str: String;

        recvfrom(*connection_socket, &mut *buffer).expect("Reading Failed");
        val_read_str = String::from_utf8_lossy(buffer.as_slice())
            .parse()
            .expect("Parsing Failed");

        println!("---Client Request---\n{}", val_read_str);
        server.logger.log("Received client request!");
        buffer
    }

    fn send_response_to_socket(server: &Arc<Server>, new_socket: RawFd, buffer: Vec<u8>) {
        let mime_response = Server::create_response(&server, buffer);

        if send(new_socket, &mime_response.as_ref(), MsgFlags::empty()).is_err() {
            println!("failed to send");
            server
                .error_service
                .serve_500_response("Sending failed".to_string());
        };
        server.logger.log("Response sent");
        println!("------------------Response sent-------------------\n");
    }

    fn create_response(server: &Arc<Server>, buffer: Vec<u8>) -> Vec<u8> {
        let mut request = request::Request::new(buffer);
        let response = server.base_controller.execute_request(&mut request);

        let mut mime_response = MimeResponse {
            http_status_code: response.http_status_code,
            content_type: response.content_type,
            content_length: response.content_bytes.len(),
        };

        let built_mime_response = mime_response.build_mime_response();

        let mime_res_ref: &[u8] = built_mime_response.as_ref();
        let mut mime_res_vec = mime_res_ref.to_vec();
        mime_res_vec.extend(response.content_bytes);
        mime_res_vec
    }
}

mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use chrono::prelude::*;
    use hyper::{Client, Uri};
    //use std::env;
    #[cfg(test)]
    use std::fs::read_to_string;
    //use std::thread;

    #[tokio::main]
    async fn client() {
        let client = Client::new();

        let url: Uri = "http://localhost:8080/index.html".parse().unwrap();

        match client.get(url).await {
            Ok(res) => println!("Response: {}", res.status()),
            Err(err) => println!("Error: {}", err),
        }
    }

    #[test]
    fn listen() {
        client();
        let now: DateTime<Local> = Local::now();
        let complete_message = format!("{:?}  [Server]: Server started listening.\n", now);
        read_to_string("resources/logs/test_log.txt").expect("Unable to read file to String");
    }
}
