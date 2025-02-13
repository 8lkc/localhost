mod config;
mod epoll;

use std::{collections::HashMap, io::{Read, Write}, os::fd::AsRawFd};
use libc::{epoll_ctl, epoll_event, epoll_wait, EPOLL_CTL_ADD, EPOLLIN, EPOLLOUT};
use mio::net::{TcpListener, TcpStream};

use self::config::Config;

pub struct Localhost {
    config: Config,
}

impl Localhost {
    pub fn new() -> Self {Localhost {config: Config::new()}}

    // pub fn start(&self) {
    //     let epoll_fd = unsafe {libc::epoll_create1(0)};
    //     if epoll_fd < 0 {panic!("Error creating epoll descriptor")}

    //     if self.config.servers.is_empty() {panic!("No server configuration found")}

    //     let addresses = self.config.servers.iter().map(|config| {
    //         config.ports.iter().map(|port| {format!("{}:{}", config.host, port)}).collect::<Vec<String>>()
    //     }).flatten().collect::<Vec<String>>();
    //     let mut servers = HashMap::new();
    //     let mut clients = HashMap::new();

    //     for address in &addresses {
    //         let std_listener = std::net::TcpListener::bind(address).expect("Unable to start server");
    //         std_listener.set_nonblocking(true).expect("Cannot set non-blocking");
    //         let listener = TcpListener::from_std(std_listener);
    
    //         let fd = listener.as_raw_fd();
    //         let mut event = libc::epoll_event {
    //             events: EPOLLIN as u32,
    //             u64: fd as u64,
    //         };
    
    //         unsafe {
    //             if epoll_ctl(epoll_fd, EPOLL_CTL_ADD, fd, &mut event) < 0 {
    //                 panic!("Error adding file descriptor to epoll");
    //             }
    //         }
    //         servers.insert(fd, listener);
    //     }
    //     println!("Servers listening on {:#?}", addresses);

    //     let mut events: [epoll_event; 1024] = unsafe { std::mem::zeroed() };
    //     loop {
    //         let num_events = unsafe { epoll_wait(epoll_fd, events.as_mut_ptr(), events.len() as i32, -1) };
    //         if num_events < 0 {panic!("ERROR: epoll_wait")}

    //         for i in 0..num_events as usize {
    //             let fd = events[i].u64 as i32;

    //             if let Some(listener) = servers.get(&fd) {
    //                 match listener.accept() {
    //                     Ok((stream, address)) => {
    //                         println!("New connection from: {:?}", address);
    //                         let stream_fd = stream.as_raw_fd();
    //                         clients.insert(stream_fd, stream);

    //                         let mut event = libc::epoll_event {
    //                             events: (EPOLLIN | EPOLLOUT) as u32,
    //                             u64: stream_fd as u64,
    //                         };

    //                         unsafe {
    //                             if epoll_ctl(epoll_fd, EPOLL_CTL_ADD, stream_fd, &mut event) < 0 {
    //                                 panic!("Error adding client file descriptor to epoll");
    //                             }
    //                         }
    //                     }
    //                     Err(ref error) if error.kind() == std::io::ErrorKind::WouldBlock => {continue}
    //                     Err(error) => {eprintln!("Error accepting connection: {:?}", error)}
    //                 }
    //             } else if let Some(stream) = clients.get_mut(&fd) {Self::handle_client(&mut *stream)}
    //         }
    //     }
    // }

    // fn handle_client(stream: &mut TcpStream) {
    //     let mut buffer = [0; 1024];
    //     match stream.read(&mut buffer) {
    //         Ok(_) => {}
    //         Err(ref error) if error.kind() == std::io::ErrorKind::WouldBlock => {return}
    //         Err(error) => {
    //             eprintln!("Error reading from stream: {:?}", error);
    //             return;
    //         }
    //     }
    //     let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    //     stream.write_all(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }
}
