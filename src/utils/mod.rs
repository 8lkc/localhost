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
        generate_session_id,
        get_current_timestamp,
        get_last_cleanup,
        has_valid_session,
        init_files,
        process_cgi_output,
        process_header_line,
        process_req_line,
        read_buffer,
        read_sessions,
        update_last_cleanup,
        write_sessions,
    },
    globals::{
        INTERPRETERS,
        SESSION_STORE,
        TEMPLATES,
        TIMEOUT,
    },
};
