use crate::{loader::Config, Server};
use libc::{epoll_ctl, epoll_event, epoll_wait, EPOLLIN, EPOLL_CTL_ADD};
use std::{
    collections::HashMap,
    net::TcpListener,
    os::{fd::AsRawFd, unix::io::RawFd},
};

#[derive(Debug)]
pub struct Multiplexer {
    epoll_fd: RawFd,
    servers: Vec<Server>,
    listeners: Vec<TcpListener>,
}

impl Multiplexer {
    pub fn new(config: Config) -> Result<Self, String> {
        let servers = config.servers();
        let epoll_fd = unsafe { libc::epoll_create1(0) };
        if epoll_fd == -1 {
            return Err(std::io::Error::last_os_error().to_string());
        };

        let mut mux_listeners = vec![];

        for server in &servers {
            match server.listeners() {
                Ok(server_listeners) => mux_listeners.push(server_listeners),
                Err(error) => return Err(error.to_string()),
            }
        }

        let listeners = mux_listeners
            .into_iter()
            .flatten()
            .collect();

        Ok(Self {
            epoll_fd,
            servers,
            listeners,
        })
    }

    pub fn epoll_fd(&self) -> RawFd {
        self.epoll_fd
    }

    pub fn servers(&self) -> &Vec<Server> {
        &self.servers
    }

    pub fn add_fd(&self) -> Result<(), String> {
        for listener in self.listeners.iter() {
            let fd = listener.as_raw_fd();

            listener
                .set_nonblocking(true)
                .map_err(|e| e.to_string())?;

            let mut event: epoll_event = unsafe { std::mem::zeroed() };
            event.events = EPOLLIN as u32;
            event.u64 = fd as u64;

            if unsafe {
                epoll_ctl(
                    self.epoll_fd,
                    libc::EPOLL_CTL_ADD,
                    fd,
                    &mut event,
                )
            } < 0
            {
                return Err(std::io::Error::last_os_error().to_string());
            }
        }
        Ok(())
    }

    pub fn run(&self) {
        let mut events: Vec<epoll_event> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            let nfds = unsafe {
                libc::epoll_wait(
                    self.epoll_fd,
                    events.as_mut_ptr(),
                    events.len() as i32,
                    -1,
                )
            };

            if nfds < 0 {
                dbg!(std::io::Error::last_os_error().to_string());
                continue;
            }

            for n in 0..nfds as usize {
                let fd = events[n].u64 as RawFd;
                
                for listener in self.listeners.iter() {
                    if listener.as_raw_fd() == fd {
                        match listener.accept() {
                            Ok((stream, addr)) => {},
                            Err(error) => {
                                dbg!(error);
                                continue
                            }
                        }
                    }
                }
            }
        }
    }
}
