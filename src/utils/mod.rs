mod errors;
mod functions;

pub(super) use {
    errors::{
        AppErr,
        AppResult,
    },
    functions::{
        process_header_line,
        process_req_line,
    },
};
