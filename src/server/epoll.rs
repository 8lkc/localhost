use libc;
use mio::net::{TcpListener, TcpStream};
use std::{io, os::fd::AsRawFd};

pub struct Event {
    is_readable: bool,
    is_writable: bool,
    is_error: bool,
    is_hangup: bool
} impl Event {
    // checkers
    pub fn is_readable(&self) -> bool { self.is_readable }
    pub fn is_writable(&self) -> bool { self.is_writable }
    pub fn is_error(&self) -> bool { self.is_error }
    pub fn is_hangup(&self) -> bool { self.is_hangup }
}

pub struct Epoll {
    fd: i32,
    events: Vec<libc::epoll_event>,
    max_events: usize,
    timeout: i32,
} impl Drop for Epoll {
    fn drop(&mut self) { unsafe { libc::close(self.fd); }}
} impl Epoll {
    pub fn new(max_events: usize, timeout_ms: i32) -> io::Result<Self> {
        let fd = unsafe { libc::epoll_create1(0) };
        if fd < 0 { return Err(io::Error::last_os_error()); }

        Ok(Self {
            fd,
            events: vec![unsafe { std::mem::zeroed() }; max_events],
            max_events,
            timeout: timeout_ms,
        })
    }

    pub fn add_listener(&self, listener: &TcpListener) -> io::Result<()> {
        let fd = listener.as_raw_fd();
        let event = libc::epoll_event {
            events: (libc::EPOLLIN | libc::EPOLLET) as u32,
            u64: fd as u64,
        };
        self.check_operation(libc::EPOLL_CTL_ADD, fd, event)?;
        Ok(())
    }

    pub fn add_stream(&self, stream: &TcpStream) -> io::Result<()> {
        let fd = stream.as_raw_fd();
        let event = libc::epoll_event {
            events: (libc::EPOLLIN | libc::EPOLLOUT | libc::EPOLLET | libc::EPOLLRDHUP) as u32,
            u64: fd as u64,
        };
        self.check_operation(libc::EPOLL_CTL_ADD, fd, event)?;
        Ok(())
    }

    pub fn modify_stream(&self, stream: &TcpStream, readable: bool, writable: bool) -> io::Result<()> {
        let fd = stream.as_raw_fd();
        let mut events = libc::EPOLLET as u32 | libc::EPOLLRDHUP as u32;
        if readable { events |= libc::EPOLLIN as u32; }
        if writable { events |= libc::EPOLLOUT as u32; }

        let event = libc::epoll_event {
            events,
            u64: fd as u64,
        };
        self.check_operation(libc::EPOLL_CTL_MOD, fd, event)?;
        Ok(())
    }

    pub fn remove_fd(&self, fd: i32) -> io::Result<()> {
        self.check_operation(libc::EPOLL_CTL_DEL, fd, libc::epoll_event {
            events: 0,
            u64: fd as u64,
        })?;
        Ok(())
    }

    pub fn wait(&mut self) -> io::Result<Vec<(i32, Event)>> {
        let num_events = unsafe { libc::epoll_wait(
            self.fd,
            self.events.as_mut_ptr(),
            self.max_events as i32,
            self.timeout,
        )};
        if num_events < 0 { return Err(io::Error::last_os_error()); }

        let mut ready_events = Vec::with_capacity(num_events as usize);
        for i in 0..num_events as usize {
            let event = self.events[i];
            let fd = event.u64 as i32;
            let flags = event.events;

            ready_events.push((fd, Event {
                is_readable: (flags & libc::EPOLLIN as u32) != 0,
                is_writable: (flags & libc::EPOLLOUT as u32) != 0,
                is_error: (flags & libc::EPOLLERR as u32) != 0,
                is_hangup: (flags & (libc::EPOLLRDHUP as u32 | libc::EPOLLHUP as u32)) != 0,
            }));
        }

        Ok(ready_events)
    }

    fn check_operation(&self, indicator: libc::c_int, fd: i32, mut event: libc::epoll_event) -> io::Result<()> {
        if unsafe { libc::epoll_ctl(self.fd, indicator, fd, &mut event) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

// pub struct EpollInstance {
//     epoll: Epoll,
//     servers: HashMap<i32, TcpListener>,
//     clients: HashMap<i32, TcpStream>,
// } impl EpollInstance {
//     pub fn new() -> io::Result<Self> {
//         Ok(Self {
//             epoll: Epoll::new(1024, -1)?,
//             servers: HashMap::new(),
//             clients: HashMap::new(),
//         })
//     }

//     pub fn install(addresses: &[String], mut handler: impl FnMut(&mut TcpStream)) -> io::Result<()> {
//         let mut instance = Self::new()?;

//         // Set up listeners
//         for address in addresses {
//             let std_listener = std::net::TcpListener::bind(address)?;
//             std_listener.set_nonblocking(true)?;
//             let listener = TcpListener::from_std(std_listener);
//             let fd = listener.as_raw_fd();
            
//             instance.epoll.add_listener(&listener)?;
//             instance.servers.insert(fd, listener);
//             println!("Server listening on {}", address);
//         }

//         loop {
//             let events = instance.epoll.wait()?;
            
//             for (fd, event) in events {
//                 if instance.servers.contains_key(&fd) {
//                     if event.is_readable {
//                         if let Some(listener) = instance.servers.get(&fd) {
//                             match listener.accept() {
//                                 Ok((stream, addr)) => {
//                                     println!("New connection from: {}", addr);
//                                     let stream_fd = stream.as_raw_fd();
//                                     instance.epoll.add_stream(&stream)?;
//                                     instance.clients.insert(stream_fd, stream);
//                                 }
//                                 Err(ref output) if output.kind() == io::ErrorKind::WouldBlock => continue,
//                                 Err(output) => eprintln!("Accept error: {}", output),
//                             }
//                         }
//                     }
//                 } else if let Some(stream) = instance.clients.get_mut(&fd) {
//                     if event.is_hangup || event.is_error {
//                         instance.epoll.remove_fd(fd)?;
//                         instance.clients.remove(&fd);
//                         continue;
//                     }
//                     if event.is_readable || event.is_writable { handler(stream) }
//                 }
//             }
//         }
//     }
// }
