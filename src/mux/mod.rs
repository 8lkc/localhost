mod add;
mod run;

#[cfg(target_os = "linux")]
use libc::epoll_create1;
#[cfg(target_os = "macos")]
use libc::kqueue;
use {
    crate::{
        loader::Config,
        server::Server,
        utils::{
            AppErr,
            AppResult,
        },
    },
    std::{
        net::{
            TcpListener,
            TcpStream,
        },
        os::fd::RawFd,
    },
};

pub struct Multiplexer {
    fd:        RawFd,
    servers:   Vec<Server>,
    listeners: Vec<TcpListener>,
    streams:   Vec<TcpStream>,
}

impl Multiplexer {
    /// Initializes a new `Multiplexer` from the given loaded
    /// configuration file.
    pub fn new(config: Config) -> AppResult<Self> {
        let servers = config.servers(); //----            ---> Retrieves servers from the configuration.

        // Creates a new kernel event queue.
        #[cfg(target_os = "linux")]
        let fd = unsafe { epoll_create1(0) }; //                   <--- Initialize epoll for Linux
        #[cfg(target_os = "macos")]
        let fd = unsafe { kqueue() }; //                   <--- Initialize kqueue for macOS

        if fd == -1 {
            return Err(AppErr::last_os_error());
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
            fd,
            servers,
            listeners,
            streams: vec![],
        })
    }
}
