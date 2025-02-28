#[macro_use]
mod macros;
mod errors;
mod functions;
mod globals;

pub use functions::cleanup_sessions;
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
        generate_session_id,
        has_valid_session,
        process_header_line,
        process_req_line,
        read_buffer,
    },
    globals::{
        INTERPRETERS,
        SESSION_STORE,
        TEMPLATES,
        TIMEOUT,
    },
};
