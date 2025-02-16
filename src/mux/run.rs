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
        syscall,
        utils::timeout,
        Request,
    },
    std::{
        ffi::c_void,
        io::{
            BufRead,
            BufReader,
            Error,
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
        // Set a vector of potentially uninitialized kevents
        // with specified capacity.
        let mut events: Vec<MaybeUninit<kevent>> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        // Start the main process.
        loop {
            dbg!("Running...");

            let number_of_found_descriptors = match syscall!(
                kevent,
                self.file_descriptor,
                null(),
                0,
                events.as_mut_ptr() as *mut kevent,
                events.len() as i32,
                timeout(1000)
            ) {
                Ok(n) => n,
                Err(e) => {
                    dbg!(e);
                    continue;
                }
            };

            // for n in 0..number_of_found_descriptors as usize {
            for event in events
                .iter()
                .take(number_of_found_descriptors as usize)
            {
                let event = unsafe { event.assume_init() };
                let fd = event.ident as RawFd;

                for listener in self.listeners.iter() {
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
                            Err(ref e)
                                if e.kind() == ErrorKind::WouldBlock =>
                            {
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
                            self.file_descriptor,
                            &changes,
                            1,
                            null_mut(),
                            0,
                            null(),
                        )
                    } < 0
                    {
                        dbg!(Error::last_os_error().to_string());
                    }
                }
            }
        }
    }
}
