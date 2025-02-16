mod errors;
mod functions;

#[macro_use]
mod macros;

pub(super) use {
    errors::{
        AppErr,
        AppResult,
    },
    functions::{
        get_listeners,
        process_header_line,
        process_req_line,
        timeout,
    },
};
