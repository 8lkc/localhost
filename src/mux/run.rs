#[cfg(target_os = "linux")]
use libc::epoll_event;
use libc::{EPOLLET, EPOLLIN, EPOLL_CTL_ADD};
use {
    super::Multiplexer,
    crate::{
        server::{cgi::CommonGatewayInterface, router::Router},
        syscall, Request,
    },
    std::{
        io::{BufRead, BufReader, Error, ErrorKind},
        mem::MaybeUninit,
        os::fd::{AsRawFd, RawFd},
    },
};
#[cfg(target_os = "macos")]
use {
    crate::utils::timeout,
    libc::{kevent, EVFILT_READ, EV_ADD},
    std::{
        ffi::c_void,
        ptr::{null, null_mut},
    },
};

impl Multiplexer {
    pub fn run(&self) {
        // Set a vector of potentially uninitialized events
        // with specified capacity.
        #[cfg(target_os = "linux")]
        let mut events: Vec<MaybeUninit<epoll_event>> =
            Vec::with_capacity(32);
        #[cfg(target_os = "macos")]
        let mut events: Vec<MaybeUninit<kevent>> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        // Start the main process.
        loop {
            #[cfg(target_os = "linux")]
            let result = syscall!(
                epoll_wait,
                self.file_descriptor,
                events.as_mut_ptr() as *mut epoll_event,
                events.len() as i32,
                1000,
            );
            #[cfg(target_os = "macos")]
            let result = syscall!(
                kevent,
                self.file_descriptor,
                null(),
                0,
                events.as_mut_ptr() as *mut kevent,
                events.len() as i32,
                timeout(1000)
            );
            let number_of_found_descriptors = match result {
                Ok(n) => n,
                Err(e) => {
                    dbg!(e);
                    continue;
                }
            };

            dbg!(number_of_found_descriptors);

            // for n in 0..number_of_found_descriptors as usize {
            for event in events
                .iter()
                .take(number_of_found_descriptors as usize)
            {
                dbg!("Checking for events...");

                let event = unsafe { event.assume_init() };
                #[cfg(target_os = "macos")]
                let fd = event.ident as RawFd;
                #[cfg(target_os = "linux")]
                let fd = event.u64 as RawFd;

                for listener in self.listeners.iter() {
                    dbg!("Checking for listeners...");

                    if listener.as_raw_fd() != fd {
                        continue;
                    }

                    let (mut stream, addr) = match listener.accept() {
                        Ok((stream, addr)) => (stream, addr),
                        Err(error) => {
                            dbg!(error);
                            continue;
                        }
                    };

                    if let Err(e) = stream.set_nonblocking(true) {
                        dbg!(e);
                        continue;
                    }

                    dbg!(&stream, addr);

                    let stream_fd = stream.as_raw_fd();
                    let mut buf_reader = BufReader::new(&stream);
                    let mut req_str = String::new();

                    'buf_reading: loop {
                        dbg!("Reading buffer...");

                        let mut line = String::new();
                        match buf_reader.read_line(&mut line) {
                            Ok(0) => break 'buf_reading,
                            Ok(_) => {
                                req_str.push_str(&line);

                                if line == "\r\n" || line == "\n" {
                                    break 'buf_reading;
                                }
                            }
                            Err(ref e)
                                if e.kind() == ErrorKind::WouldBlock =>
                            {
                                // Try again since it would block

                                dbg!("Blocking Read!");
                                continue;
                            }
                            Err(e) => {
                                dbg!(e);
                                break 'buf_reading;
                            }
                        }

                        dbg!("Finish reading buffer!");
                    }

                    if req_str.is_empty() {
                        dbg!("Empty Request...");
                        continue;
                    }

                    let request = Request::from(req_str);
                    let cgi = CommonGatewayInterface;

                    match cgi.is_cgi_request(&request, &self.servers) {
                        Ok(Some(cgi_py)) => {
                            dbg!("Run CGI...");
                            let cgi_script = cgi.execute_cgi(
                                &cgi_py,
                                &request,
                                &mut stream,
                            );

                            if let Err(e) = cgi_script {
                                dbg!(e);
                                continue;
                            }
                        }
                        Ok(None) => {
                            dbg!("Run Router...");
                            if let Err(e) =
                                Router::run(request, &mut stream)
                            {
                                dbg!(e);
                                continue;
                            }
                        }
                        Err(error) => {
                            dbg!(error);
                            continue;
                        }
                    };

                    #[cfg(target_os = "linux")]
                    let mut event = epoll_event {
                        events: EPOLLIN as u32 | EPOLLET as u32,
                        u64: stream_fd as u64,
                    };
                    #[cfg(target_os = "macos")]
                    let changes = kevent {
                        ident: stream_fd as usize,
                        filter: EVFILT_READ,
                        flags: EV_ADD,
                        fflags: 0,
                        data: 0,
                        udata: null_mut::<c_void>(),
                    };

                    #[cfg(target_os = "linux")]
                    let result = syscall!(
                        epoll_ctl,
                        self.file_descriptor,
                        EPOLL_CTL_ADD,
                        stream_fd,
                        &mut event,
                    );
                    #[cfg(target_os = "macos")]
                    let result = syscall!(
                        kevent,
                        self.file_descriptor,
                        &changes,
                        1,
                        null_mut(),
                        0,
                        null(),
                    );

                    if let Err(_) = result {
                        dbg!(Error::last_os_error().to_string());
                    }
                }
            }
        }
    }
}
