use crate::controller::base_controller::BaseController;
use crate::controller::controller::Controller;
use crate::enums::content_type::ContentType;
use crate::server::mime_response::MimeResponse;
use crate::utils::logger::Logger;
use image::ImageFormat;
use libc::in_addr;
use libc::INADDR_ANY;
use nix::sys::socket::*;
use nix::unistd::close;
use std::os::unix::io::RawFd;
use crate::enums::http_status_codes;
use crate::enums::http_status_codes::HTTPStatusCodes;

const PORT: u16 = 8087;

pub struct Server {
    server_fd: RawFd,
    logger: Logger,
    base_controller: BaseController,
}

impl Server {
    pub fn new() -> Server {
        let server_fd = Server::setup();
        let logger = Logger::new(String::from("Server"));
        let base_controller = BaseController::new();
        Server {
            server_fd,
            logger,
            base_controller,
        }
    }

    pub fn listen(&mut self, backlog: usize) {
        self.logger.log("Server started listening.");
        listen(self.server_fd, backlog).expect("Listening Failed");

        loop {
            let (connection_socket, val_read_string) = self.read_incoming_connection();
            self.send_response_to_socket(connection_socket, val_read_string);
            close(connection_socket).expect("Closing Failed");
        }
    }

    fn setup() -> RawFd {
        let server_fd = socket(
            AddressFamily::Inet,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )
        .expect("Unable to create socket.");

        let in_address = in_addr { s_addr: INADDR_ANY };

        let sockaddr_in = nix::sys::socket::sockaddr_in {
            sin_len: 255,
            sin_family: libc::AF_INET as u8,
            sin_port: PORT.to_be(),
            sin_addr: in_address,
            sin_zero: [0; 8],
        };

        bind(server_fd, &SockAddr::Inet(InetAddr::V4(sockaddr_in))).expect("Binding Failed");
        server_fd
    }

    fn read_incoming_connection(&mut self) -> (RawFd, String) {
        println!("\n+++++++ Waiting for new connection ++++++++\n\n");

        let new_socket = accept(self.server_fd).expect("Accepting Failed");
        let mut buffer = vec![0; 30000];
        let val_read_str: String;

        recvfrom(new_socket, &mut *buffer).expect("Reading Failed");
        val_read_str = String::from_utf8_lossy(buffer.as_slice())
            .parse()
            .expect("Parsing Failed");

        println!("---Client Request---\n{}", val_read_str);
        self.logger.log("Received client request!");
        (new_socket, val_read_str)
    }

    fn send_response_to_socket(&mut self, new_socket: RawFd, val_read_str: String) {
        let mut mime_response = self.create_response(val_read_str);

        send(new_socket, &mime_response.as_ref(), MsgFlags::empty()).expect("Sending Failed");
        println!("------------------Response sent-------------------\n");
    }

    fn create_response(&mut self, val_read_str: String) -> Vec<u8> {
        let route_path = val_read_str
            .split_whitespace()
            .nth(1)
            .expect("Unable to split result");

        let content_result = self.base_controller.serve_content(route_path);
        let content_type = self.base_controller.get_content_type_for_path(route_path);

        let (content, http_status_code) = match content_result {
            Ok(ok_content) => (ok_content, HTTPStatusCodes::Ok),
            Err(err_content) => (err_content, HTTPStatusCodes::NotFound),
        };
        println!("CONTENT TYPE: {}", content_type.as_str());
        let mut mime_response = MimeResponse {
            http_status_code,
            content_type,
            content_length: content.len(),
        };

        let builded_mime_response = mime_response.build_mime_response();

        let mut mime_res_ref: &[u8] = builded_mime_response.as_ref();
        let mut mime_res_vec =  mime_res_ref.to_vec();
        mime_res_vec.extend(content);
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
