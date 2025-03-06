mod add;
mod core;
mod poll;
mod read;
mod remove;
mod run;

#[cfg(target_os = "linux")]
use libc::epoll_event;
#[cfg(target_os = "macos")]
use libc::kevent;
#[cfg(target_os = "windows")]
use windows::Win32::System::IO::OVERLAPPED;
use {
    crate::server::Server,
    std::{
        collections::HashMap,
        net::{
            TcpListener,
            TcpStream,
        },
        os::fd::RawFd,
    },
};

#[cfg(target_os = "linux")]
pub type OsEvent = epoll_event;
#[cfg(target_os = "macos")]
pub type OsEvent = kevent;
#[cfg(target_os = "windows")]
pub type OsEvent = OVERLAPPED;

/// Manages connection:
/// - Accepts incoming connections through TCP listeners
/// - Reading HTTP requests using non-blocking I/O
/// - Dispatches requests to the appropriate server
/// - Processes requests and sends responses back through a router system
pub struct Multiplexer {
    file_descriptor: RawFd,
    servers:         Vec<Server>,
    listeners:       Vec<(TcpListener, usize)>,
    streams:         HashMap<RawFd, Client>,
}

pub(self) struct Client {
    stream:     TcpStream,
    req_buf:    Vec<u8>,
    server_idx: usize,
    content:    Option<usize>,
}

impl Client {
    pub fn new(stream: TcpStream, server_idx: usize) -> Self {
        Self {
            stream,
            req_buf: Vec::new(),
            server_idx,
            content: None,
        }
    }

    pub fn set_content_len(&mut self, len: usize) { self.content = Some(len) }
}
