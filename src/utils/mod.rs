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
        has_valid_session,
        process_header_line,
        process_req_line,
        read_buffer,
        get_current_timestamp,
        init_files,
        read_sessions,
        write_sessions,
        get_last_cleanup,
        update_last_cleanup
    },
    globals::{
        INTERPRETERS,
        SESSION_STORE,
        TEMPLATES,
        TIMEOUT,
    },
};
