#[cfg(target_os = "linux")]
use {
    super::{ErrorAddFd, Multiplexer},
    crate::{
        http::Request,
        loader::Config,
        server::{cgi::CommonGatewayInterface, router::Router, Server},
    },
    libc::{epoll_create1, epoll_ctl, epoll_event, epoll_wait, EPOLLIN, EPOLL_CTL_ADD},
    std::{
        io::{BufRead, BufReader, Error, ErrorKind},
        os::{fd::AsRawFd, unix::io::RawFd},
        ptr::null_mut,
    },
};

#[cfg(target_os = "linux")]
impl Multiplexer {
    pub fn new(config: Config) -> Result<Self, String> {
        let servers = config.servers();
        let epoll_fd = unsafe { epoll_create1(0) };
        if epoll_fd == -1 {
            return Err(Error::last_os_error().to_string());
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
            epoll_fd,
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

            let mut event = epoll_event {
                events: EPOLLIN as u32,
                u64: fd as u64,
            };

            if unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, fd, &mut event) } < 0 {
                return Err(Error::last_os_error().to_string());
            }
        }
        Ok(())
    }

    pub fn run(&self) {
        let mut events: Vec<epoll_event> = vec![epoll_event { events: 0, u64: 0 }; 32];

        loop {
            dbg!("Start Multiplexer...");
            let nfds =
                unsafe { epoll_wait(self.epoll_fd, events.as_mut_ptr(), events.len() as i32, -1) };

            if nfds < 0 {
                dbg!(Error::last_os_error().to_string());
                continue;
            }

            for event in events.iter().take(nfds as usize) {
                let fd = event.u64 as RawFd;

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

                                            if line == "\r\n" || line == "\n" {
                                                break 'buf_reading;
                                            }
                                        }
                                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
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

                                let mut event = epoll_event {
                                    events: EPOLLIN as u32,
                                    u64: stream_fd as u64,
                                };

                                if unsafe {
                                    epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, stream_fd, &mut event)
                                } < 0
                                {
                                    dbg!(Error::last_os_error().to_string());
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
