use {
    super::{
        ClientConnectionState,
        Multiplexer,
    },
    crate::debug,
    std::{
        io::{
            ErrorKind,
            Read,
            Write,
        },
        net::Shutdown,
        os::fd::{AsRawFd, RawFd},
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

                        let client_state = ClientConnectionState::new(stream, *server_idx);
                        self.streams
                            .insert(stream_fd, client_state);
                    }
                    None => {
                        if let Some(mut client_state) = self.streams.remove(&event_fd) {
                            let client_fd = event_fd;
                            if self.can_read(&event) {
                                let mut buf = [0u8; 1024];
                                match client_state
                                    .stream
                                    .read(&mut buf)
                                {
                                    Ok(..=0) => {
                                        if let Err(e) = self.remove(client_fd) {
                                            debug!(e);
                                            continue;
                                        }
                                    }
                                    Ok(bytes) => {
                                        client_state
                                            .req_buf
                                            .extend_from_slice(&buf[..bytes]);

                                        let request =
                                            &String::from_utf8_lossy(&client_state.req_buf)
                                                .to_string()
                                                .into();

                                        let response: String = self.servers
                                            [client_state.server_idx]
                                            .router()
                                            .direct(request)
                                            .into();

                                        if let Err(e) = client_state
                                            .stream
                                            .write(response.as_bytes())
                                        {
                                            debug!(e);
                                            if let Err(e) = client_state
                                                .stream
                                                .shutdown(Shutdown::Both)
                                            {
                                                debug!(e);
                                            };
                                        };
                                    }
                                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                                        self.streams
                                            .insert(client_fd, client_state);
                                    }
                                    Err(e) => {
                                        debug!(e);
                                        if let Err(e) = client_state
                                            .stream
                                            .shutdown(Shutdown::Both)
                                        {
                                            debug!(e);
                                        }
                                        if let Err(e) = self.remove(client_fd) {
                                            debug!(e);
                                        }
                                        continue;
                                    }
                                }
                            }
                        }
                        continue;
                    }
                };
            }
        }
    }
}
