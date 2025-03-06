#[macro_use]
mod macros;
mod errors;
mod functions;
mod globals;
// mod chunk;

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
        get_current_timestamp,
        get_session_id,
        process_cgi_output,
        process_header_line,
        process_req_line,
    },
    globals::{
        BOUNDARY_REGEX,
        CONTENT_DISPOSITION_REGEX,
        CONTENT_TYPE_REGEX,
        HTTP,
        INTERPRETERS,
        TEMPLATES,
        TIMEOUT,
    },
};
