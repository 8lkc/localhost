#[cfg(target_os = "linux")]
use libc::{
    epoll_event,
    EPOLLET,
    EPOLLIN,
    EPOLL_CTL_ADD,
};
#[cfg(target_os = "windows")]
use windows::Win32::System::IO::{
    CreateIoCompletionPort,
    GetQueuedCompletionStatus,
    INVALID_HANDLE_VALUE,
    OVERLAPPED,
};
use {
    super::{
        Multiplexer,
        OsEvent,
    },
    crate::{
        syscall,
        utils::{
            AppResult,
            TIMEOUT,
        },
    },
    std::os::fd::RawFd,
};
#[cfg(target_os = "macos")]
use {
    crate::utils::timeout,
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
    pub(super) fn register(&self, fd: RawFd) -> AppResult<i32> {
        #[cfg(target_os = "linux")]
        {
            let mut event = epoll_event {
                events: EPOLLIN as u32 | EPOLLET as u32,
                u64:    fd as u64,
            };

            syscall!(
                epoll_ctl,
                self.file_descriptor,
                EPOLL_CTL_ADD,
                fd,
                &mut event
            )
        }
        #[cfg(target_os = "macos")]
        {
            let event = kevent {
                ident:  fd as usize,
                filter: EVFILT_READ,
                flags:  EV_ADD,
                fflags: 0,
                data:   0,
                udata:  null_mut::<c_void>(),
            };

            syscall!(
                kevent,
                self.file_descriptor,
                &event,
                1,
                null_mut(),
                0,
                null(),
            )
        }
        #[cfg(target_os = "windows")]
        {
            syscall!(
                CreateIoCompletionPort,
                fd as HANDLE,
                self.file_descriptor,
                0,
                0
            )
        }
    }

    pub(super) fn poll(
        &self,
        events: &mut Vec<OsEvent>,
    ) -> AppResult<i32> {
        #[cfg(target_os = "linux")]
        {
            syscall!(
                epoll_wait,
                self.file_descriptor,
                events.as_mut_ptr() as *mut epoll_event,
                events.len() as i32,
                TIMEOUT as i32,
            )
        }
        #[cfg(target_os = "macos")]
        {
            syscall!(
                kevent,
                self.file_descriptor,
                null(),
                0,
                events.as_mut_ptr() as *mut kevent,
                events.len() as i32,
                timeout(TIMEOUT)
            )
        }
        #[cfg(target_os = "windows")]
        {
            let mut bytes_transferred = 0;
            let mut completion_key = 0;
            let mut overlapped = null_mut();

            syscall!(
                GetQueuedCompletionStatus,
                self.file_descriptor,
                &mut bytes_transferred,
                &mut completion_key,
                &mut overlapped,
                TIMEOUT
            )
        }
    }
}
