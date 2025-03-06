use {
    super::{
        Client,
        Multiplexer,
    },
    crate::{
        debug,
        message::Request,
    },
    std::{
        io::{
            ErrorKind,
            Read,
            Write,
        },
        net::Shutdown,
        os::fd::{
            AsRawFd,
            RawFd,
        },
    },
};

impl Multiplexer {
    /// Starts the main process by setting a vector of potentially
    /// uninitialized events with a specified capacity. Then gets the file
    /// descriptor from the event through the number of found descriptors
    /// (nfds), finds the listener associated with the file descriptor,
    /// gets the stream and address from the associated listener and makes
    /// the stream asynchronous. Then from the stream buffer, gets
    /// the request, adds the stream file descriptor to the
    /// `Multiplexer`and finally sends the `Request` to the `Router`.
    pub fn run(&mut self) {
        let mut events = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            let nfds = match self.poll(&mut events) {
                Ok(nfds) => nfds as usize,
                Err(e) => {
                    debug!(e);
                    continue;
                }
            };

            for event in events.iter().take(nfds) {
                let event = unsafe { event.assume_init() };

                #[cfg(target_os = "linux")]
                let event_fd = event.u64 as RawFd;
                #[cfg(target_os = "macos")]
                let event_fd = event.ident as RawFd;
                #[cfg(target_os = "windows")]
                let event_fd = event.fd as RawFd;

                match self.find_listener(event_fd) {
                    Some((listener, server_idx)) => {
                        let (stream, _addr) = match listener.accept() {
                            Ok((stream, addr)) => (stream, addr),
                            Err(e) => {
                                debug!(e);
                                continue;
                            }
                        };

                        if let Err(e) = stream.set_nonblocking(true) {
                            debug!(e);
                            if let Err(e) = stream.shutdown(Shutdown::Both) {
                                debug!(e);
                            };
                            continue;
                        };

                        let stream_fd = stream.as_raw_fd();
                        if let Err(e) = self.add(stream_fd) {
                            debug!(e);
                            if let Err(e) = stream.shutdown(Shutdown::Both) {
                                debug!(e);
                            };
                            continue;
                        };

                        let client = Client::new(stream, *server_idx);
                        self.streams
                            .insert(stream_fd, client);
                    }
                    None => {
                        if !self.can_read(&event) {
                            continue;
                        }

                        if let Some(client) = self
                            .streams
                            .get_mut(&event_fd)
                        {
                            let client_fd = event_fd;
                            let mut buf = [0u8; 1024];
                            match client.stream.read(&mut buf) {
                                Ok(0) => {
                                    if let Err(e) = self.remove(client_fd) {
                                        debug!(e);
                                        continue;
                                    }
                                }
                                Ok(bytes) => {
                                    client
                                        .req_buf
                                        .extend_from_slice(&buf[..debug!(bytes)]);

                                    let req_str =
                                        String::from_utf8_lossy(&client.req_buf).to_string();

                                    match req_str
                                        .lines()
                                        .find(|line| line.contains("Content-Length"))
                                    {
                                        Some(line) => {
                                            if client.content.is_none() {
                                                client.set_content_len(
                                                    line.split(": ")
                                                        .nth(1)
                                                        .unwrap_or("0")
                                                        .parse()
                                                        .unwrap_or(0),
                                                )
                                            }
                                        }
                                        None => {}
                                    };

                                    if client.content.is_some()
                                        && client.req_buf.len() <= client.content.unwrap()
                                    {
                                        continue;
                                    }

                                    let request: Request = req_str.into();

                                    let response: String = self.servers[client.server_idx]
                                        .router()
                                        .direct(request)
                                        .into();

                                    if let Err(e) = client
                                        .stream
                                        .write(response.as_bytes())
                                    {
                                        debug!(e);
                                        if let Err(e) = client
                                            .stream
                                            .shutdown(Shutdown::Both)
                                        {
                                            debug!(e);
                                        };
                                    };

                                    client.req_buf.clear();
                                }
                                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                                    continue;
                                }
                                Err(e) => {
                                    debug!(e);
                                    if let Err(e) = client
                                        .stream
                                        .shutdown(Shutdown::Both)
                                    {
                                        debug!(e);
                                    }
                                    if let Err(e) = self.remove(client_fd) {
                                        debug!(e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
