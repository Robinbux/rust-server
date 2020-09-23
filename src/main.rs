use nix::sys::socket::*;
use nix::sys::uio::IoVec;
use nix::unistd::close;
use libc::in_addr;
use libc::INADDR_ANY;
use std::mem::ManuallyDrop;
use std::ptr;
use fancy_regex::Regex;
use std::fs::{File, read_to_string};
use std::io::Read;

const PORT:u16 = 8090;

fn main() {
    let hello = "HTTP/1.1 200 OK\nContent-Type: text/html\nContent-Length: 500\n\n";

    let server_fd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None
    ).expect("Unable to create socket.");

    let in_address = in_addr {
        s_addr: INADDR_ANY
    };

    let sockaddr_in = nix::sys::socket::sockaddr_in {
        sin_len: 255,
        sin_family: libc::AF_INET as u8,
        sin_port: PORT.to_be(),
        sin_addr: in_address,
        sin_zero: [0; 8]
    };


    bind(server_fd, &SockAddr::Inet(InetAddr::V4(sockaddr_in)))
        .expect("Binding Failed");

    listen(server_fd, 10)
        .expect("Listening Failed");

    loop {
        println!("\n+++++++ Waiting for new connection ++++++++\n\n");

        let new_socket = accept(server_fd)
            .expect("Accepting Failed");

        let mut buffer: [u8; 30000] = [0; 30000];
        let vec_buffer = ManuallyDrop::new(IoVec::from_mut_slice(&mut buffer));

        let mut val_read_str: String;

        unsafe {
            recvmsg(
                new_socket,
                &[ptr::read(&*vec_buffer)],
                None,
                MsgFlags::empty()
            ).expect("Reading Failed");
            val_read_str = String::from_utf8_lossy(vec_buffer.as_slice()).parse()
                .expect("Parsing Failed");
        }

        println!("---Client Message---\n{}", val_read_str);


        let result = val_read_str.split_whitespace().nth(1);

        let path = format!("resources{}", result.unwrap());

        let html_string = read_to_string(path);

        let mime_response = format!("{}{}", hello, html_string.unwrap());


        send(
            new_socket,
            mime_response.as_ref(),
            MsgFlags::empty()
        ).expect("Sending Failed");

        println!("------------------Hello message sent-------------------\n");

        close(new_socket).expect("Closing Failed");
    }
}
