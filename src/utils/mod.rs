mod errors;
mod functions;

#[macro_use]
mod macros;

#[cfg(target_os = "macos")]
pub(super) use functions::timeout;
pub(super) use {
    errors::{
        AppErr,
        AppResult,
    },
    functions::{
        get_listeners,
        process_header_line,
        process_req_line,
    },
};
