// #[cfg(target_os = "macos")]
use {
     super::Multiplexer,
     crate::{
          http::Request,
          loader::Config,
          server::{
               cgi::CommonGatewayInterface,
               router::Router,
               Server,
          },
          utils::{
               Err,
               Result,
          },
     },
     libc::{
          c_void,
          kevent,
          kqueue,
          EVFILT_READ,
          EV_ADD,
     },
     std::{
          io::{
               BufRead,
               BufReader,
               Error,
               ErrorKind,
          },
          mem::MaybeUninit,
          os::{
               fd::AsRawFd,
               unix::io::RawFd,
          },
          ptr::{
               null,
               null_mut,
          },
     },
};

// #[cfg(target_os = "macos")]
impl Multiplexer {
     /// Initializes a new `Multiplexer` from the given loaded
     /// configuration file.
     pub fn new(config: Config) -> Result<Self> {
          let servers = config.servers(); //----            ---> Retrieves servers from the configuration.

          // Creates a new kernel event queue.
          let kqueue_fd = unsafe { kqueue() };
          if kqueue_fd == -1 {
               return Err(Err::NonBlocking(Error::new(
                    ErrorKind::WouldBlock,
                    "Failed to create the event queue!",
               )));
          };

          // Collects each server's ports listeners.
          let mut mux_listeners = vec![];
          for server in &servers {
               match server.listeners() {
                    Ok(server_listeners) => {
                         mux_listeners.push(server_listeners)
                    }
                    Err(e) => return Err(e),
               }
          }

          // Flattens all listeners.
          let listeners = mux_listeners
               .into_iter()
               .flatten()
               .collect();

          Ok(Self {
               kqueue_fd,
               servers,
               listeners,
               streams: vec![],
          })
     }

     pub fn epoll_fd(&self) -> RawFd { self.kqueue_fd }

     pub fn servers(&self) -> &Vec<Server> { &self.servers }

     /// Adds a new file descriptor for each listener.
     pub fn add_fd(&self) -> Result<()> {
          for listener in self.listeners.iter() {
               let fd = listener.as_raw_fd(); //----                        ---> Extracts the raw file descriptor.

               listener
                    .set_nonblocking(true)?; //----    ---> Moves this TCP stream into nonblocking mode.

               let changes = kevent {
                    ident:  fd as usize,
                    filter: EVFILT_READ,
                    flags:  EV_ADD,
                    fflags: 0,
                    data:   0,
                    // udata:  0 as *mut libc::c_void,
                    udata:  null_mut::<c_void>(),
               };

               if unsafe {
                    kevent(
                         self.kqueue_fd,
                         &changes,
                         1,
                         null_mut(),
                         0,
                         null(),
                    )
               } < 0
               {
                    return Err(Err::from(Error::last_os_error()));
               }
          }
          Ok(())
     }

     pub fn run(&self) {
          let mut events: Vec<MaybeUninit<kevent>> =
               Vec::with_capacity(32);
          unsafe { events.set_len(32) };

          loop {
               dbg!("Start Multiplexer...");
               let nfds = unsafe {
                    kevent(
                         self.kqueue_fd,
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

                                        let request =
                                             Request::from(req_str);
                                        let cgi = CommonGatewayInterface;

                                        match cgi.is_cgi_request(
                                             &request,
                                             &self.servers,
                                        ) {
                                             Ok(Some(cgi_py)) => {
                                                  dbg!("Run CGI...");
                                                  let cgi_script = cgi
                                                       .execute_cgi(
                                                            &cgi_py,
                                                            &request,
                                                            &mut stream,
                                                       );

                                                  if let Err(e) =
                                                       cgi_script
                                                  {
                                                       dbg!(e);
                                                       continue;
                                                  }
                                             }
                                             Ok(None) => {
                                                  dbg!("Run Router...");
                                                  if let Err(e) =
                                                       Router::run(
                                                            request,
                                                            &mut stream,
                                                       )
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
                                                  self.kqueue_fd,
                                                  &changes,
                                                  1,
                                                  null_mut(),
                                                  0,
                                                  null(),
                                             )
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
