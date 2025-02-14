#[cfg(target_os = "macos")]
use libc::{
    kevent,
    EVFILT_READ,
    EV_ADD,
};
use {
    super::Multiplexer,
    crate::{
        server::{
            cgi::CommonGatewayInterface,
            router::Router,
        },
        utils::AppErr,
        Request,
    },
    std::{
        ffi::c_void,
        io::{
            BufRead,
            BufReader,
            ErrorKind,
        },
        mem::MaybeUninit,
        os::fd::{
            AsRawFd,
            RawFd,
        },
        ptr::{
            null,
            null_mut,
        },
    },
};

impl Multiplexer {
    pub fn run(&self) {
        let mut events: Vec<MaybeUninit<kevent>> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            dbg!("Start Multiplexer...");
            let nfds = unsafe {
                kevent(
                    self.fd,
                    std::ptr::null(),
                    0,
                    events.as_mut_ptr() as *mut kevent,
                    events.len() as i32,
                    std::ptr::null(),
                )
            };

            if nfds < 0 {
                dbg!(AppErr::last_os_error());
                continue;
            }

            // for n in 0..nfds as usize {
            for event in events
                .iter()
                .take(nfds as usize)
            {
                let event = unsafe { event.assume_init() };
                let fd = event.ident as RawFd;

                for listener in self.listeners.iter() {
                    if listener.as_raw_fd() == fd {
                        match listener.accept() {
                            Ok((mut stream, addr)) => {
                                if let Err(e) =
                                    stream.set_nonblocking(true)
                                {
                                    dbg!(e);
                                    continue;
                                }

                                dbg!(&stream, addr);

                                let stream_fd = stream.as_raw_fd();
                                let mut buf_reader =
                                    BufReader::new(&stream);
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

                                match cgi.is_cgi_request(
                                    &request,
                                    &self.servers,
                                ) {
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
                                        if let Err(e) = Router::run(
                                            request,
                                            &mut stream,
                                        ) {
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
                                    ident:  stream_fd as usize,
                                    filter: EVFILT_READ,
                                    flags:  EV_ADD,
                                    fflags: 0,
                                    data:   0,
                                    udata:  null_mut::<c_void>(),
                                };

                                if unsafe {
                                    kevent(
                                        self.fd,
                                        &changes,
                                        1,
                                        null_mut(),
                                        0,
                                        null(),
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
