#[cfg(target_os = "macos")]
use {
    super::Multiplexer,
    crate::{
        http::Request,
        loader::Config,
        server::{
            cgi::CommonGatewayInterface,
            Server,
        },
    },
    libc::{
        kevent,
        kqueue,
        EVFILT_READ,
        EV_ADD,
    },
    std::{
        io::{
            BufRead,
            BufReader,
        },
        os::{
            fd::AsRawFd,
            unix::io::RawFd,
        },
    },
};

use super::ErrorAddFd;

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
                Ok(server_listeners) => {
                    mux_listeners.push(server_listeners)
                }
                Err(error) => return Err(error.to_string()),
            }
        }

        let listeners = mux_listeners
            .into_iter()
            .flatten()
            .collect();

        Ok(Self {
            epoll_fd: kqueue_fd,
            servers,
            listeners,
            streams: vec![],
        })
    }

    pub fn epoll_fd(&self) -> RawFd { self.epoll_fd }

    pub fn servers(&self) -> &Vec<Server> { &self.servers }

    pub fn add_fd(&self) -> ErrorAddFd {
        for listener in self.listeners.iter() {
            let fd = listener.as_raw_fd();

            listener
                .set_nonblocking(true)
                .map_err(|e| e.to_string())?;

            let changes = kevent {
                ident:  fd as usize,
                filter: EVFILT_READ,
                flags:  EV_ADD,
                fflags: 0,
                data:   0,
                udata:  0 as *mut libc::c_void,
            };

            if unsafe {
                kevent(
                    self.epoll_fd,
                    &changes,
                    1,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null(),
                )
            } < 0
            {
                return Err(std::io::Error::last_os_error().to_string());
            }
        }
        Ok(())
    }

    pub fn run(&self) {
        let mut events: Vec<kevent> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            let nfds = unsafe {
                kevent(
                    self.epoll_fd,
                    std::ptr::null(),
                    0,
                    events.as_mut_ptr(),
                    events.len() as i32,
                    std::ptr::null(),
                )
            };

            if nfds < 0 {
                dbg!(std::io::Error::last_os_error().to_string());
                continue;
            }

            for n in 0..nfds as usize {
                let fd = events[n].ident as RawFd;

                for listener in self.listeners.iter() {
                    if listener.as_raw_fd() == fd {
                        match listener.accept() {
                            Ok((mut stream, addr)) => {
                                if let Err(e) = stream
                                    .set_nonblocking(true)
                                    .map_err(|e| e.to_string())
                                {
                                    dbg!(e);
                                    continue;
                                }
                                dbg!(&stream, addr);
                                let stream_fd = stream.as_raw_fd();
                                let buf_reader = BufReader::new(&stream);
                                let mut request_string = String::new();
                                for line in buf_reader.lines() {
                                    match line {
                                        Ok(line) => {
                                            request_string.push_str(&line);
                                            request_string.push_str("\n");
                                        }
                                        Err(error) => {
                                            dbg!(error);
                                            continue;
                                        }
                                    }
                                }

                                let request =
                                    Request::from(request_string);
                                let cgi = CommonGatewayInterface;

                                match cgi.is_cgi_request(
                                    &request,
                                    &self.servers,
                                ) {
                                    Ok(Some(cgi_py)) => {
                                        let cgi_script = cgi.execute_cgi(
                                            &cgi_py,
                                            &request,
                                            &mut stream,
                                        );
                                        if let Err(error) = cgi_script {
                                            dbg!(error);
                                            continue;
                                        }
                                    }
                                    Ok(None) => {
                                        // let router = Router;
                                        // router.route(request, &mut
                                        // stream);
                                        continue;
                                    }
                                    Err(error) => {
                                        dbg!(error);
                                        continue;
                                    }
                                };

                                let changes = kevent {
                                    ident:  stream_fd as usize,
                                    filter: EVFILT_READ,
                                    flags:  EV_ADD,
                                    fflags: 0,
                                    data:   0,
                                    udata:  0 as *mut libc::c_void,
                                };

                                if unsafe {
                                    kevent(
                                        self.epoll_fd,
                                        &changes,
                                        1,
                                        std::ptr::null_mut(),
                                        0,
                                        std::ptr::null(),
                                    )
                                } < 0
                                {
                                    dbg!(std::io::Error::last_os_error()
                                        .to_string());
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
