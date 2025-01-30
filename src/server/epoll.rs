use std::collections::HashMap;
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::{Duration, Instant};
use std::net::TcpStream;

const MAX_EVENTS: usize = 1024;
const TIMEOUT_MS: i32 = 30_000; // 30 seconds timeout
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

pub struct EpollHandler {
    epoll_fd: RawFd,
    events: Vec<libc::epoll_event>,
    connections: HashMap<RawFd, Connection>,
}

pub struct Connection {
    pub stream: TcpStream,
    pub created_at: Instant,
    pub buffer: Vec<u8>,
    pub write_buffer: Vec<u8>,
}

impl EpollHandler {
    pub fn new() -> io::Result<Self> {
        let epoll_fd = unsafe { libc::epoll_create1(0) };
        if epoll_fd == -1 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            epoll_fd,
            events: vec![libc::epoll_event { events: 0, u64: 0 }; MAX_EVENTS],
            connections: HashMap::new(),
        })
    }
    pub fn add_fd(&self, fd: RawFd, events: u32) -> io::Result<()> {
        let event = libc::epoll_event {
            events,
            u64: fd as u64,
        };
        let ret = unsafe { libc::epoll_ctl(self.epoll_fd, libc::EPOLL_CTL_ADD, fd, &event as *const _ as *mut _) };

        if ret == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub fn get_connection_mut(&mut self, fd: RawFd) -> Option<&mut Connection> {
        self.connections.get_mut(&fd)
    }

    pub fn add_socket(&mut self, socket: TcpStream, events: u32) -> io::Result<()> {
        let fd = socket.as_raw_fd();
        let mut event = libc::epoll_event {
            events,
            u64: fd as u64,
        };

        let result = unsafe {
            libc::epoll_ctl(
                self.epoll_fd,
                libc::EPOLL_CTL_ADD,
                fd,
                &mut event as *mut libc::epoll_event,
            )
        };

        if result == -1 {
            return Err(io::Error::last_os_error());
        }

        self.connections.insert(fd, Connection {
            stream: socket,
            created_at: Instant::now(),
            buffer: Vec::with_capacity(4096),
            write_buffer: Vec::new(),
        });

        Ok(())
    }

    pub fn modify_socket(&mut self, fd: RawFd, events: u32) -> io::Result<()> {
        let mut event = libc::epoll_event {
            events,
            u64: fd as u64,
        };

        let result = unsafe {
            libc::epoll_ctl(
                self.epoll_fd,
                libc::EPOLL_CTL_MOD,
                fd,
                &mut event as *mut libc::epoll_event,
            )
        };

        if result == -1 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn remove_socket(&mut self, fd: RawFd) -> io::Result<()> {
        let result = unsafe {
            libc::epoll_ctl(
                self.epoll_fd,
                libc::EPOLL_CTL_DEL,
                fd,
                std::ptr::null_mut(),
            )
        };

        if result == -1 {
            return Err(io::Error::last_os_error());
        }

        self.connections.remove(&fd);
        Ok(())
    }

    pub fn wait(&mut self) -> io::Result<Vec<(RawFd, u32)>> {
        let n = unsafe {
            libc::epoll_wait(
                self.epoll_fd,
                self.events.as_mut_ptr(),
                MAX_EVENTS as i32,
                TIMEOUT_MS,
            )
        };

        if n == -1 {
            return Err(io::Error::last_os_error());
        }

        let mut ready_events = Vec::new();
        for i in 0..n as usize {
            let event = self.events[i];
            ready_events.push((event.u64 as RawFd, event.events));
        }

        Ok(ready_events)
    }

    pub fn check_timeouts(&mut self) -> io::Result<()> {
        let now = Instant::now();
        let expired: Vec<RawFd> = self.connections
            .iter()
            .filter(|(_, conn)| now.duration_since(conn.created_at) > CONNECTION_TIMEOUT)
            .map(|(&fd, _)| fd)
            .collect();

        for fd in expired {
            self.remove_socket(fd)?;
        }

        Ok(())
    }
}

impl Drop for EpollHandler {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.epoll_fd);
        }
    }
}