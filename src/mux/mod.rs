#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod mac_os;

use {
    crate::server::Server,
    std::{
        net::{
            TcpListener,
            TcpStream,
        },
        os::fd::RawFd,
    },
};

pub struct Multiplexer {
    #[cfg(target_os = "linux")]
    epoll_fd: RawFd,

    #[cfg(target_os = "macos")]
    kqueue_fd: RawFd,

    servers:   Vec<Server>,
    listeners: Vec<TcpListener>,
    streams:   Vec<TcpStream>,
}
