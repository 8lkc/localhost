#[macro_use]
mod macros;
mod constants;
mod errors;
mod functions;

#[cfg(target_os = "macos")]
pub(super) use functions::timeout;
pub(super) use {
    constants::TIMEOUT,
    errors::{
        AppErr,
        AppResult,
    },
    functions::{
        get_listeners,
        process_header_line,
        process_req_line,
        read_buffer,
    },
};
