use std::{fs, io, net::TcpListener, path::Path};

use libc;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {servers:Vec<Server>}
impl Config {
    pub fn from_file<P: AsRef<Path>>(path:P) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config:Config = toml::from_str(&contents)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        Ok(config)
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

    pub(super) fn install(&self, listeners:&Vec<ListenerInfo>) -> io::Result<()> {
        // Register each listener's file descriptor with the epoll instance.
        for listener_info in listeners {
            let fd = listener_info.get_file_descriptor();
            let mut event = libc::epoll_event {events: (libc::EPOLLIN | libc::EPOLLET) as u32, u64: fd as u64};
            if unsafe {
                libc::epoll_ctl(self.file_descriptor, libc::EPOLL_CTL_ADD, fd, &mut event as *mut libc::epoll_event)
            } < 0 {return Err(io::Error::last_os_error());}
        }

        // Buffer to hold events (we assume a maximum of 10 events at a time).
        let mut events = vec![libc::epoll_event { events: 0, u64: 0 }; 10];

        loop {
            // Wait indefinitely for events.
            let nfds = unsafe {
                libc::epoll_wait(self.file_descriptor, events.as_mut_ptr(), events.len() as i32, -1)
            };
            if nfds < 0 {return Err(io::Error::last_os_error());}

            // Process each triggered event.
            for n in 0..(nfds as usize) {
                let ev = events[n];
                let event_fd = ev.u64 as i32;
                // Find the corresponding listener.
                if let Some(listener_info) = listeners.iter().find(|l| l.file_descriptor == event_fd) {
                    // Accept all pending connections.
                    loop {
                        match listener_info.listener.accept() {
                            Ok((stream, addr)) => {
                                println!("Accepted connection from {} on server '{}'", addr, listener_info.server_name);
                                // Future: Register client streams with epoll for further I/O.
                                drop(stream); // For now, we simply drop the connection.
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                            Err(err) => {
                                eprintln!("Error accepting connection on server '{}': {}", listener_info.server_name, err);
                                break;
                            }
                        }
                    }
                } else {println!("Received event on unknown fd: {}", event_fd)}
            }
        }
    }
}

// Holds information for a single listener.
pub(super) struct ListenerInfo {server_name:String, listener:TcpListener, file_descriptor:i32}
impl ListenerInfo {
    pub(super) fn new(server_name:String, listener:TcpListener, fd:i32) -> Self {
        Self {server_name, listener, file_descriptor: fd}
    }
    // Getters
    pub(super) fn get_file_descriptor(&self) -> i32 {self.file_descriptor}
}

#[derive(Debug, Deserialize)]
pub struct Server {name:String, host:String, ports:Vec<u16>}
impl Server {
    // Just for the getters
    pub fn get_host(&self) -> &str {&self.host}
    pub fn get_name(&self) -> &str {&self.name}
    pub fn get_ports(&self) -> &[u16] {&self.ports}
}
