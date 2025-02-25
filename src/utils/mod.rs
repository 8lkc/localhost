#[macro_use]
mod macros;
mod errors;
mod functions;
mod globals;

#[cfg(target_os = "macos")]
pub(super) use functions::timeout;
pub(super) use {
    errors::{
        AppErr,
        AppResult,
        HttpErr,
        HttpResult,
    },
    functions::{
        get_listeners,
        process_header_line,
        process_req_line,
        read_buffer,
    },
    globals::{
        INTERPRETERS,
        TEMPLATES,
        TIMEOUT,
    },
};
