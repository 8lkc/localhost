use {
    super::{
        Multiplexer,
        OsEvent,
    },
    crate::debug,
    std::{
        io::{
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
    pub fn run(&self) {
        let mut events: Vec<OsEvent> = Vec::with_capacity(32);
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
                let mut buf = [0u8; 1024];

                #[cfg(target_os = "linux")]
                let event_fd = event.u64 as RawFd;
                #[cfg(target_os = "macos")]
                let event_fd = event.ident as RawFd;
                #[cfg(target_os = "windows")]
                let event_fd = event.fd as RawFd;

                let (fd_listener, server_idx) =
                    match self.find_listener(event_fd) {
                        Some(pair) => pair,
                        None => continue,
                    };

                let (mut stream, _addr) = match fd_listener.accept() {
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

                if let Err(e) = self.add(stream.as_raw_fd()) {
                    debug!(e);
                    if let Err(e) = stream.shutdown(Shutdown::Both) {
                        debug!(e);
                    };
                    continue;
                };

                let request = match stream.read(&mut buf) {
                    Ok(_) => String::from_utf8_lossy(&buf[..])
                        .to_string()
                        .into(),
                    Err(e) => {
                        debug!(e);
                        if let Err(e) = stream.shutdown(Shutdown::Both) {
                            debug!(e);
                        };
                        continue;
                    }
                };

                let response: String = self.servers[*server_idx]
                    .router()
                    .direct(request)
                    .into();

                if let Err(e) = stream.write(response.as_bytes()) {
                    debug!(e);
                    if let Err(e) = stream.shutdown(Shutdown::Both) {
                        debug!(e);
                    };
                };
            }
        }
    }
}
