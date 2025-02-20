use {
    super::{
        Multiplexer,
        OsEvent,
    },
    crate::{
        server::{
            cgi::Cgi,
            router::Router,
        },
        utils::{
            read_buffer,
            AppErr,
        },
        Request,
    },
    std::os::fd::{
        AsRawFd,
        RawFd,
    },
};

impl Multiplexer {
    pub fn run(&self) -> ! {
        // Set a vector of potentially uninitialized events
        // with specified capacity.
        let mut events: Vec<OsEvent> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        // Start the main process.
        loop {
            // Number of found descriptors
            let nfds = self
                .poll(&mut events)
                .unwrap_or(0) as usize;

            dbg!(nfds);

            for event in events.iter().take(nfds) {
                let event = unsafe { event.assume_init() };

                // Get the file descriptor from the event.
                #[cfg(target_os = "linux")]
                let event_fd = event.u64 as RawFd;
                #[cfg(target_os = "macos")]
                let event_fd = event.ident as RawFd;

                // Find the listener associated with the file descriptor.
                let fd_listener = match self.find_listener(event_fd) {
                    Some(listener) => listener,
                    None => continue,
                };

                // Get the stream and address from the associated listener.
                let (mut stream, addr) = match fd_listener.accept() {
                    Ok((stream, addr)) => (stream, addr),
                    Err(_) => continue,
                };

                if stream
                    .set_nonblocking(true)
                    .is_err()
                {
                    continue;
                }

                dbg!(addr);

                // Get the request from the stream.
                let request = match read_buffer(&stream) {
                    Some(req_str) => Request::from(req_str),
                    None => continue,
                };

                let cgi = Cgi;
                match cgi.is_cgi_request(&request, &self.servers) {
                    Ok(script) => {
                        dbg!("Run CGI...");
                        let cgi_script =
                            cgi.execute(&script, &request, &mut stream);

                        if let Err(e) = cgi_script {
                            dbg!(e);
                            continue;
                        }
                    }
                    Err(AppErr::NoCGI) => {
                        if let Err(e) =
                            Router::direct(request, &mut stream)
                        {
                            dbg!(e);
                            continue;
                        }
                    }
                    Err(e) => {
                        dbg!(e);
                        continue;
                    }
                };

                if let Err(e) = self.register(stream.as_raw_fd()) {
                    dbg!(e);
                }
            }
        }
    }
}
