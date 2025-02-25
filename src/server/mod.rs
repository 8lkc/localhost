mod epoll;
pub mod config;

use std::{
    collections::HashMap,
    fs::File,
    {io::{self, BufRead}, os::fd::AsRawFd},
    time::Duration
};
use config::Server;
use epoll::Epoll;
use mio::net::TcpListener;

use crate::http::connection::ConnectionState;
use crate::Connection;

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_EVENTS: usize = 1024;
const EPOLL_TIMEOUT: i32 = 1000; // 1 second timeout for epoll_wait

pub struct Localhost {
    addresses: Vec<String>,
    connections: HashMap<i32, Connection>,
    epoll: Epoll,
}

impl Localhost {
    pub fn new(servers: &[Server]) -> io::Result<Self> {
        Ok(Self {
            addresses: servers
                .iter()
                .flat_map(|server| {
                    server
                        .get_ports()
                        .iter()
                        .map(move |port| format!("{}:{}", server.get_host(), port))
                })
                .collect(),
            connections: HashMap::new(),
            epoll: Epoll::new(MAX_EVENTS, EPOLL_TIMEOUT)?,
        })
    }

    pub fn start(&mut self) -> io::Result<()> {
        Self::header()?;

        // Set up listeners
        let mut listeners = Vec::new();
        for address in &self.addresses {
            let std_listener = std::net::TcpListener::bind(address)?;
            std_listener.set_nonblocking(true)?;
            let listener = TcpListener::from_std(std_listener);
            let fd = listener.as_raw_fd();
            
            self.epoll.add_listener(&listener)?;
            listeners.push((fd, listener));
            println!("Server listening on {}", address);
        }

        loop {
            // Handle events
            let events = self.epoll.wait()?;
            
            for (fd, event) in events {
                // Handle listener events
                if let Some((_, listener)) = listeners.iter().find(|(lfd, _)| *lfd == fd) {
                    if event.is_readable() {
                        self.handle_new_connection(listener)?;
                    }
                    continue;
                }

                // Handle client events
                if let Some(conn) = self.connections.get_mut(&fd) {
                    if event.is_hangup() || event.is_error() {
                        self.remove_connection(fd)?;
                        continue;
                    }

                    if event.is_readable() && conn.get_state() == &ConnectionState::Reading {
                        if let Err(e) = conn.handle_readable() {
                            eprintln!("Error reading from connection: {}", e);
                            self.remove_connection(fd)?;
                            continue;
                        }
                    }

                    if event.is_writable() && conn.get_state() == &ConnectionState::Writing {
                        if let Err(e) = conn.handle_writable() {
                            eprintln!("Error writing to connection: {}", e);
                            self.remove_connection(fd)?;
                            continue;
                        }
                    }
                }
            }

            // Check for timeouts
            let timed_out: Vec<i32> = self.connections
                .iter()
                .filter(|(_, conn)| conn.is_timed_out(CONNECTION_TIMEOUT))
                .map(|(fd, _)| *fd)
                .collect();

            for fd in timed_out {
                println!("Connection timed out: {}", fd);
                self.remove_connection(fd)?;
            }
        }
    }

    fn handle_new_connection(&mut self, listener: &TcpListener) -> io::Result<()> {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New connection from: {}", addr);
                let fd = stream.as_raw_fd();
                self.epoll.add_stream(&stream)?;
                let connection = Connection::new(stream);
                self.connections.insert(fd, connection);
                Ok(())
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn remove_connection(&mut self, fd: i32) -> io::Result<()> {
        self.epoll.remove_fd(fd)?;
        self.connections.remove(&fd);
        Ok(())
    }

    fn header() -> io::Result<()> {
        let file = File::open("assets/header.txt")?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() { println!("\t{}", line?) }
        Ok(())
    }
}
