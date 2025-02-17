#[macro_export]
macro_rules! check {
    ($result:expr) => {{
        if $result == -1 {
            Err($crate::utils::AppErr::last_os_error())
        }
        else {
            Ok($result)
        }
    }};
}

#[macro_export]
macro_rules! syscall {
    ($name:ident $(, $arg:expr)* $(,)?) => {{
        $crate::check!(unsafe { libc::$name($($arg),*) })
    }};
}
