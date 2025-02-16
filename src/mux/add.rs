#[cfg(target_os = "linux")]
use libc::{
    epoll_event,
    EPOLLET,
    EPOLLIN,
    EPOLL_CTL_ADD,
};
use {
    super::Multiplexer,
    crate::{
        syscall,
        utils::AppResult,
    },
    std::os::fd::AsRawFd,
};
#[cfg(target_os = "macos")]
use {
    libc::{
        kevent,
        EVFILT_READ,
        EV_ADD,
    },
    std::{
        ffi::c_void,
        ptr::{
            null,
            null_mut,
        },
    },
};

impl Multiplexer {
    /// Adds a new file descriptor for each listener.
    pub fn add_fd(&self) -> AppResult<()> {
        for listener in self.listeners.iter() {
            let fd = listener.as_raw_fd(); //----                        ---> Extracts the raw file descriptor.

            listener.set_nonblocking(true)?; //----                  ---> Moves each socket into nonblocking mode.

            #[cfg(target_os = "linux")]
            let mut event = epoll_event {
                events: EPOLLIN as u32 | EPOLLET as u32,
                u64: fd as u64,
            };
            #[cfg(target_os = "macos")]
            let event = kevent {
                ident: fd as usize,
                filter: EVFILT_READ,
                flags: EV_ADD,
                fflags: 0,
                data: 0,
                udata: null_mut::<c_void>(),
            };

            #[cfg(target_os = "linux")]
            syscall!(
                epoll_ctl,
                self.file_descriptor,
                EPOLL_CTL_ADD,
                fd,
                &mut event
            )?;
            #[cfg(target_os = "macos")]
            syscall!(
                kevent,
                self.file_descriptor,
                &event,
                1,
                null_mut(),
                0,
                null(),
            )?;
        }
        Ok(())
    }
}
