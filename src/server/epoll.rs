pub(super) struct Epoll {
    file_descriptor: i32
}

impl Epoll {
    fn new() -> Self {
        let epoll_fd = unsafe {libc::epoll_create1(0)};
        if epoll_fd < 0 {panic!("Error creating epoll descriptor")}
        Self {file_descriptor: epoll_fd}
    }

    pub(super) fn setup() {
    }
}
