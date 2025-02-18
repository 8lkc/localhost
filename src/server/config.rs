use std::{collections::HashMap, io, path::Path};

use mio::net::{TcpListener, TcpStream};
use serde::Deserialize;
use std::os::fd::AsRawFd;

#[derive(Deserialize)]
pub struct Config {servers:Vec<Server>}
impl Config {
    pub fn from_file<P:AsRef<Path>>(path:P) -> Self {
        let config = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config).expect("ERROR")
    }

    pub fn get_servers(&self) -> &[Server] {&self.servers}
}

pub(super) struct EpollInstance {file_descriptor:i32}
impl EpollInstance {
    pub(super) fn new() -> Self {
        let epoll_fd = unsafe {libc::epoll_create1(0)};
        if epoll_fd < 0 {panic!("ERROR: {}", io::Error::last_os_error())}
        Self {file_descriptor: epoll_fd}
    }

    pub(super) fn install(addresses:&[String], handler:fn(&mut TcpStream)) {
        let epoll_instance = EpollInstance::new();
        let mut servers = HashMap::new();
        let mut clients = HashMap::new();

        for addresse in addresses {
            let std_listener = std::net::TcpListener::bind(addresse).expect("Unable to start server");
            std_listener.set_nonblocking(true).expect("Cannot set non-blocking");
            let listener = TcpListener::from_std(std_listener);

            let file_descriptor = listener.as_raw_fd();
            let mut event = libc::epoll_event {
                events: libc::EPOLLIN as u32,
                u64: file_descriptor as u64,
            };

            unsafe {
                if libc::epoll_ctl(epoll_instance.file_descriptor, libc::EPOLL_CTL_ADD, file_descriptor, &mut event) < 0 {
                    panic!("Error adding file descriptor to epoll");
                }
            }
            servers.insert(file_descriptor, listener);
        }
        println!("Servers listening on {:#?}", addresses);

        let mut events: [libc::epoll_event; 1024] = unsafe {std::mem::zeroed()};

        loop {
            let num_events = unsafe {libc::epoll_wait(
                epoll_instance.file_descriptor,
                events.as_mut_ptr(),
                events.len() as i32,
                -1,
            )};
            if num_events < 0 {panic!("ERROR: epoll_wait")}

            for i in 0..num_events as usize {
                let fd = events[i].u64 as i32;
    
                if let Some(listener) = servers.get(&fd) {
                    match listener.accept() {
                        Ok((stream, address)) => {
                            println!("New connection from: {:?}", address);
                            let stream_fd = stream.as_raw_fd();
                            clients.insert(stream_fd, stream);
    
                            let mut event = libc::epoll_event {
                                events: (libc::EPOLLIN | libc::EPOLLOUT) as u32,
                                u64: stream_fd as u64,
                            };
    
                            unsafe {
                                if libc::epoll_ctl(epoll_instance.file_descriptor, libc::EPOLL_CTL_ADD, stream_fd, &mut event) < 0 {
                                    panic!("Error adding client file descriptor to epoll");
                                }
                            }
                        }
                        Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {continue}
                        Err(err) => {eprintln!("Error accepting connection: {:?}", err)}
                    }
                } else if let Some(stream) = clients.get_mut(&fd) {handler(&mut *stream)}
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Server {name:String, host:String, ports:Vec<u16>}
impl Server {
    // Just for the getters
    pub fn get_host(&self) -> &str {&self.host}
    pub fn get_name(&self) -> &str {&self.name}
    pub fn get_ports(&self) -> &[u16] {&self.ports}
}
