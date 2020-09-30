use libc::in_addr;
use libc::INADDR_ANY;
use nix::sys::socket::*;
use std::os::unix::io::RawFd;
use std::fs;
use crate::mime_response::MimeResponse;
use std::fs::read_to_string;
use nix::unistd::close;

const PORT: u16 = 8080;

pub struct Server {
    server_fd: RawFd
}

impl Server {
    pub fn new() -> Server {
        let server_fd = Server::setup();
        Server {server_fd}
    }

    pub fn listen(self, backlog: usize) {
        listen(self.server_fd, backlog).expect("Listening Failed");

        loop {
            let mut connection_socket = self.read_incoming_connection();
            self.send_response_to_socket(&mut connection_socket);
            close(connection_socket).expect("Closing Failed");
        }
    }

    fn setup()-> RawFd {
        let server_fd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
        ).expect("Unable to create socket.");

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

    fn send_response_to_socket(self, new_socket: RawFd) {
        let route_path = val_read_str
            .split_whitespace()
            .nth(1)
            .expect("Unable to split result");

        let content = match self.load_resource(String::from(route_path)) {
            OK(content) => content,
            Err(e) => {
                println!(e);
                return
            }
        };
        let content_type = self.get_content_type(String::from(route_path));

        let mime_response = MimeResponse {
            http_status_code: String::from("200 OK"),
            content_type: String::from(content_type),
            content: content,
        };
        self.send_message(new_socket, mime_response.build_mime_response().as_ref());
        println!("------------------Hello message sent-------------------\n");
    }

    fn read_incoming_connection(self) -> String {
        println!("\n+++++++ Waiting for new connection ++++++++\n\n");

        let new_socket = accept(self.server_fd).expect("Accepting Failed");
        let mut buffer = vec![0; 30000];
        let mut val_read_str: String;

        recvfrom(new_socket, &mut *buffer).expect("Reading Failed");
        val_read_str = String::from_utf8_lossy(buffer.as_slice())
            .parse()
            .expect("Parsing Failed");

        println!("---Client Message---\n{}", val_read_str);
        new_socket
    }

    fn send_message(self, new_socket: RawFd, buf: &[u8]) {
        send(
            new_socket,
            mime_response.build_mime_response().as_ref(),
            MsgFlags::empty(),
        ).expect("Sending Failed");
    }

    fn load_resource(self, file_path: String) -> Result<String, String> {
        let complete_resource_path = format!("resources{}", file_path);
        let file_type = self.get_file_type(file_path);
        return match file_type {
            String::from("html") => Ok(self.load_html(complete_resource_path)),
            String::from("ico") => Ok(self.load_ico(complete_resource_path)),
            _ => Err(format!("Unsupported file type: {}", file_type))
        }
    }

    fn load_html(self, html_file_path: String) -> String {
        return read_to_string(html_file_path).expect("Unable to read file to String");
    }

    fn load_ico(self, ico_file_path: String) -> String {
        let icon = fs::read(ico_file_path).expect("Unable to read icon");
        return base64::encode(&*icon);
    }

    fn get_file_type(self, path: String) -> String {
        return file_path.split(".").last().expect("Unable to split path.");
    }

    fn get_content_type(self, path: String) -> &'static str {
        return match file_path.split(".").last().expect("Unable to split path.") {
            "html" => "text/html",
            "ico" => "image/x-icon",
            _ => "text/html"
        }
    }
}