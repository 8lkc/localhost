use {
    super::{
        Multiplexer,
        OsEvent,
    },
    crate::{
        debug,
        utils::read_buffer,
        Request,
    },
    std::{
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
    /// therequest, adds the stream file desriptor to the `Multiplexer`and
    /// finally sends the `Request` to the `Router`.
    pub fn run(&self) {
        let mut events: Vec<OsEvent> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            let nfds = self
                .poll(&mut events)
                .unwrap_or(0) as usize;

            for event in events.iter().take(nfds) {
                let event = unsafe { event.assume_init() };

                #[cfg(target_os = "linux")]
                let event_fd = event.u64 as RawFd;
                #[cfg(target_os = "macos")]
                let event_fd = event.ident as RawFd;

                let (fd_listener, idx) = match self.find_listener(event_fd)
                {
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

                let request = match read_buffer(&stream) {
                    Ok(req_str) => Request::from(req_str),
                    Err(e) => {
                        debug!(e);
                        if let Err(e) = stream.shutdown(Shutdown::Both) {
                            debug!(e);
                        };
                        continue;
                    }
                };

                if let Err(e) = self.register(stream.as_raw_fd()) {
                    debug!(e);
                    if let Err(e) = stream.shutdown(Shutdown::Both) {
                        debug!(e);
                    };
                    continue;
                };

                self.servers[*idx]
                    .router()
                    .direct(request, &mut stream)
            }
        }
    }
}
