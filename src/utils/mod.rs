#[macro_use]
mod macros;
mod errors;
mod functions;
mod globals;

#[cfg(target_os = "macos")]
pub(super) use functions::timeout;
pub use functions::cleanup_sessions;
pub(super) use {
    errors::{
        AppErr,
        AppResult,
        HttpErr,
        HttpResult,
    },
    functions::{
        process_header_line,
        process_req_line,
        read_buffer,
        generate_session_id,
    },
    globals::{
        INTERPRETERS,
        TEMPLATES,
        TIMEOUT,
    },
};
