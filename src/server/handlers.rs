use std::{io, net::TcpListener, os::fd::AsRawFd};

use super::config::{EpollInstance, ListenerInfo, Server};

pub struct Localhost {listeners:Vec<ListenerInfo>}
impl Localhost {
    // Creates a new server manager by instantiating a listener for every port of every server.
    pub fn new(servers:&[Server]) -> io::Result<Self> {
        let mut listeners = Vec::new();
        for server in servers {
            for port in server.get_ports() {
                let address = format!("{}:{}", server.get_host(), port);
                let listener = TcpListener::bind(address.clone())?;
                listener.set_nonblocking(true)?;
                let file_descriptor = listener.as_raw_fd();
                println!("Server '{}' listening on {}", server.get_name(), address);
                listeners.push(ListenerInfo::new(String::from(server.get_name()), listener, file_descriptor));
            }
        }
        Ok(Self {listeners})
    }

    // Starts the server manager.
    pub fn start(&self) -> io::Result<()> {
        // Create an epoll instance.
        let epoll_instance = EpollInstance::new();
        epoll_instance.install(&self.listeners)
    }
}
