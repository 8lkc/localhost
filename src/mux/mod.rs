mod events;
mod run;

#[cfg(target_os = "linux")]
use libc::epoll_event;
#[cfg(target_os = "macos")]
use libc::kevent;
#[cfg(target_os = "windows")]
use windows::Win32::System::IO::OVERLAPPED;
use {
    crate::{
        loader::Config,
        server::Server,
        syscall,
        utils::{
            AppErr,
            AppResult,
        },
    },
    std::{
        mem::MaybeUninit,
        net::TcpListener,
        os::fd::{
            AsRawFd,
            RawFd,
        },
    },
};

//
#[cfg(target_os = "linux")]
pub type OsEvent = MaybeUninit<epoll_event>;
#[cfg(target_os = "macos")]
pub type OsEvent = MaybeUninit<kevent>;
#[cfg(target_os = "windows")]
pub type OsEvent = MaybeUninit<OVERLAPPED>;

//------------------------------------------------------------------

/// Manages connection:
/// - Accepts incoming connections through TCP listeners
/// - Reading HTTP requests using non-blocking I/O
/// - Dispatches requests to the appropriate server
/// - Processes requests and sends responses back through a router system
pub struct Multiplexer {
    file_descriptor: RawFd,
    servers:         Vec<Server>,
    listeners:       Vec<(TcpListener, usize)>,
}

impl Multiplexer {
    /// Initializes a new `Multiplexer` from the
    /// given loaded configuration file.
    /// Creates a new kernel event queue using:
    ///
    /// - `kqueue` for macOS
    /// - `epoll` for Linux
    ///
    /// These are event notification interfaces
    /// that monitor multiple file descriptors to
    /// see if I/O is possible, allowing efficient
    /// handling of multiple connections.
    pub fn new(config: Config) -> AppResult<Self> {
        let servers = match config.servers() {
            Some(servers) => servers,
            None => return Err(AppErr::NoServer),
        };

        // Creates a new kernel event queue.
        #[cfg(target_os = "linux")]
        let file_descriptor = syscall!(epoll_create1, 0)?;
        #[cfg(target_os = "macos")]
        let file_descriptor = syscall!(kqueue)?;
        #[cfg(target_os = "windows")]
        let file_descriptor = syscall!(
            CreateIoCompletionPort,
            INVALID_HANDLE_VALUE,
            0,
            0
        )?;

        Ok(Self {
            file_descriptor,
            servers,
            listeners: vec![],
        })
    }

    /// Adds a new file descriptor for each
    /// listener.
    pub fn register_listeners(&mut self) -> AppResult<()> {
        for (idx, server) in self
            .servers
            .iter()
            .enumerate()
        {
            let listeners = server.listeners()?;
            for listener in listeners {
                let fd = listener.as_raw_fd(); //----                        ---> Extracts the raw file descriptor.
                listener.set_nonblocking(true)?; //----                  ---> Moves each socket into nonblocking mode.
                self.register(fd)?;
                self.listeners.push((listener, idx));
            }
        }

        Ok(())
    }

    pub fn find_listener(
        &self,
        fd: RawFd,
    ) -> Option<&(TcpListener, usize)> {
        self.listeners
            .iter()
            .find(|(listener, _)| listener.as_raw_fd() == fd)
    }
}
