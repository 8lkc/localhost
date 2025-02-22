use {
    super::{
        Multiplexer,
        OsEvent,
    },
    crate::{
        server::router::Router,
        utils::read_buffer,
        Request,
    },
    std::os::fd::{
        AsRawFd,
        RawFd,
    },
};

impl Multiplexer {
    /// Starts the main process by setting a vector of potentially
    /// uninitialized events with a specified capacity. Then gets the file
    /// descriptor from the event, finds the listener associated with the
    /// file descriptor, gets the stream and address from the associated
    /// listener and makes the stream asynchronous. Then from the stream
    /// buffer, gets the request, adds the stream file desriptor to the
    /// `Multiplexer` and finally sends the `Request` to the `Router`.
    pub fn run(&self) -> ! {
        let mut events: Vec<OsEvent> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            let nfds = self //                      <-- Number of found descriptors.
                .poll(&mut events)
                .unwrap_or(0) as usize;

            dbg!(nfds);

            for event in events.iter().take(nfds) {
                let event = unsafe { event.assume_init() };

                #[cfg(target_os = "linux")]
                let event_fd = event.u64 as RawFd;
                #[cfg(target_os = "macos")]
                let event_fd = event.ident as RawFd;

                let fd_listener = match self.find_listener(event_fd) {
                    Some(listener) => listener,
                    None => continue,
                };

                let (mut stream, _addr) = match fd_listener.accept() {
                    Ok((stream, addr)) => (stream, addr),
                    Err(e) => {
                        dbg!(e);
                        continue;
                    }
                };

                if let Err(e) = stream.set_nonblocking(true) {
                    dbg!(e);
                    continue;
                };

                let request = match read_buffer(&stream) {
                    Some(req_str) => Request::from(req_str),
                    None => continue,
                };

                if let Err(e) = self.register(stream.as_raw_fd()) {
                    dbg!(e);
                    continue;
                };

                Router::direct(request, &mut stream)
            }
        }
    }
}
