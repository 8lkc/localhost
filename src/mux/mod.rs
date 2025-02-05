mod linux;
mod mac_os;

use {
    crate::server::Server,
    std::{
        net::{TcpListener, TcpStream},
        os::fd::RawFd,
    },
};

pub struct Multiplexer {
    epoll_fd: RawFd,
    servers: Vec<Server>,
    listeners: Vec<TcpListener>,
    streams: Vec<TcpStream>,
}

type ErrorAddFd = Result<(), String>;
