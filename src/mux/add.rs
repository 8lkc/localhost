#[cfg(target_os = "linux")]
use libc::{
    epoll_ctl,
    epoll_event,
    EPOLLET,
    EPOLLIN,
};
#[cfg(target_os = "macos")]
use libc::{
    kevent,
    EVFILT_READ,
    EV_ADD,
};
use {
    super::Multiplexer,
    crate::utils::{
        AppErr,
        AppResult,
    },
    std::{
        ffi::c_void,
        os::fd::AsRawFd,
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
               let event = epoll_event {
                    events: EPOLLIN | EPOLLET,
                    data:   fd as u64,
               };
               #[cfg(target_os = "macos")]
               let event = kevent {
                    ident:  fd as usize,
                    filter: EVFILT_READ,
                    flags:  EV_ADD,
                    fflags: 0,
                    data:   0,
                    udata:  null_mut::<c_void>(),
               };

               if unsafe {
                   #[cfg(target_os = "linux")]
                   epoll_ctl(self.fd, EPOLL_CTL_ADD, fd, &event);
                   #[cfg(target_os = "macos")]
                   kevent(
                         self.file_descriptor,
                         &event,
                         1,
                         null_mut(),
                         0,
                         null(),
                    )
               } < 0
               {
                   return Err(AppErr::last_os_error());
               }
          }
        Ok(())
    }
}
