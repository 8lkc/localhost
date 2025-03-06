use super::{
    Multiplexer,
    OsEvent,
};

impl Multiplexer {
    pub(super) fn can_read(&mut self, event: &OsEvent) -> bool {
        #[cfg(target_os = "linux")]
        return event.events & libc::EPOLLIN as u32 != 0;

        #[cfg(target_os = "macos")]
        return event.filter | libc::EVFILT_READ as i16 != 0;

        #[cfg(target_os = "windows")]
        return event.filter | libc::FD_READ_BIT != 0;
    }
}
