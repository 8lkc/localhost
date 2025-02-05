#[cfg(target_os = "macos")]
use {
    super::{ErrorAddFd, Multiplexer},
    crate::{
        http::Request,
        loader::Config,
        server::{cgi::CommonGatewayInterface, router::Router, Server},
    },
    libc::{c_void, kevent, kqueue, EVFILT_READ, EV_ADD},
    std::{
        io::{BufRead, BufReader, Error, ErrorKind},
        mem::MaybeUninit,
        os::{fd::AsRawFd, unix::io::RawFd},
        ptr::{null, null_mut},
    },
};

#[cfg(target_os = "macos")]
impl Multiplexer {
    pub fn new(config: Config) -> Result<Self, String> {
        let servers = config.servers();
        let kqueue_fd = unsafe { kqueue() };
        if kqueue_fd == -1 {
            return Err(std::io::Error::last_os_error().to_string());
        };

        let mut mux_listeners = vec![];

        for server in &servers {
            match server.listeners() {
                Ok(server_listeners) => mux_listeners.push(server_listeners),
                Err(error) => return Err(error.to_string()),
            }
        }

        let listeners = mux_listeners.into_iter().flatten().collect();

        Ok(Self {
            epoll_fd: kqueue_fd,
            servers,
            listeners,
            streams: vec![],
        })
    }

    pub fn epoll_fd(&self) -> RawFd {
        self.epoll_fd
    }

    pub fn servers(&self) -> &Vec<Server> {
        &self.servers
    }

    pub fn add_fd(&self) -> ErrorAddFd {
        for listener in self.listeners.iter() {
            let fd = listener.as_raw_fd();

            listener.set_nonblocking(true).map_err(|e| e.to_string())?;

            let changes = kevent {
                ident: fd as usize,
                filter: EVFILT_READ,
                flags: EV_ADD,
                fflags: 0,
                data: 0,
                // udata:  0 as *mut libc::c_void,
                udata: null_mut::<c_void>(),
            };

            if unsafe { kevent(self.epoll_fd, &changes, 1, null_mut(), 0, null()) } < 0 {
                return Err(std::io::Error::last_os_error().to_string());
            }
        }
        Ok(())
    }

    pub fn run(&self) {
        let mut events: Vec<MaybeUninit<kevent>> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            dbg!("Start Multiplexer...");
            let nfds = unsafe {
                kevent(
                    self.epoll_fd,
                    std::ptr::null(),
                    0,
                    events.as_mut_ptr() as *mut kevent,
                    events.len() as i32,
                    std::ptr::null(),
                )
            };

            if nfds < 0 {
                dbg!(Error::last_os_error().to_string());
                continue;
            }

            // for n in 0..nfds as usize {
            for event in events.iter().take(nfds as usize) {
                let event = unsafe { event.assume_init() };
                let fd = event.ident as RawFd;

                for listener in self.listeners.iter() {
                    if listener.as_raw_fd() == fd {
                        match listener.accept() {
                            Ok((mut stream, addr)) => {
                                if let Err(e) = stream.set_nonblocking(true) {
                                    dbg!(e);
                                    continue;
                                }

                                dbg!(&stream, addr);

                                let stream_fd = stream.as_raw_fd();
                                let mut buf_reader = BufReader::new(&stream);
                                let mut req_str = String::new();

                                'buf_reading: loop {
                                    let mut line = String::new();
                                    match buf_reader.read_line(&mut line) {
                                        Ok(0) => break 'buf_reading,
                                        Ok(_) => {
                                            req_str.push_str(&line);
                                            // line.push('\n');

                                            if line == "\r\n" || line == "\n" {
                                                break 'buf_reading;
                                            }
                                        }
                                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                                            // Try again since it would block
                                            continue;
                                        }
                                        Err(e) => {
                                            dbg!(e);
                                            break 'buf_reading;
                                        }
                                    }
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
                                        let cgi_script =
                                            cgi.execute_cgi(&cgi_py, &request, &mut stream);

                                        if let Err(e) = cgi_script {
                                            dbg!(e);
                                            continue;
                                        }
                                    }
                                    Ok(None) => {
                                        dbg!("Run Router...");
                                        if let Err(e) = Router::run(request, &mut stream) {
                                            dbg!(e);
                                            continue;
                                        }
                                    }
                                    Err(error) => {
                                        dbg!(error);
                                        continue;
                                    }
                                };

                                let changes = kevent {
                                    ident: stream_fd as usize,
                                    filter: EVFILT_READ,
                                    flags: EV_ADD,
                                    fflags: 0,
                                    data: 0,
                                    udata: null_mut::<c_void>(),
                                };

                                if unsafe {
                                    kevent(self.epoll_fd, &changes, 1, null_mut(), 0, null())
                                } < 0
                                {
                                    dbg!(std::io::Error::last_os_error().to_string());
                                }
                            }
                            Err(error) => {
                                dbg!(error);
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
