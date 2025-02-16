mod add;
mod run;

use {
    crate::{
        loader::Config,
        server::Server,
        syscall,
        utils::{
            get_listeners,
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

//------------------------------------------------------------------

pub struct Multiplexer {
    file_descriptor: RawFd,
    servers:         Vec<Server>,
    listeners:       Vec<TcpListener>,
    streams:         Vec<TcpStream>,
}

impl Multiplexer {
    /// Initializes a new `Multiplexer` from the given loaded
    /// configuration file.
    pub fn new(config: Config) -> AppResult<Self> {
        let servers = config.servers(); //                  <--- Retrieves servers from the configuration.

        // Creates a new kernel event queue.
        #[cfg(target_os = "linux")]
        let file_descriptor = syscall!(epoll_create1)?; //                   <--- Initialize epoll for Linux
        #[cfg(target_os = "macos")]
        let file_descriptor = syscall!(kqueue)?; //                   <--- Initialize kqueue for macOS

        let listeners = get_listeners(&servers)?; //        <--- Collects each server's ports listeners.

        Ok(Self {
            file_descriptor,
            servers,
            listeners,
            streams: vec![],
        })
    }
}
