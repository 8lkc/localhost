mod errors;
mod functions;

pub(super) use {
     errors::{
          Err,
          Result,
     },
     functions::{
          process_header_line,
          process_req_line,
     },
};
