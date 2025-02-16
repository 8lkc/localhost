use {
    super::AppResult,
    crate::{
        http::{
            Method,
            Resource,
        },
        server::Server,
    },
    std::net::TcpListener,
};
#[cfg(target_os = "macos")]
use {
    libc::{
        c_long,
        time_t,
        timespec,
    },
    std::time::Duration,
};

pub fn process_req_line(s: &str) -> (Method, Resource) {
    let mut words = s.split_whitespace();

    let method = words.next().unwrap();
    let resource = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
    )
}

pub fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':');
    let key = header_items
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    let value = header_items
        .collect::<Vec<&str>>()
        .join(":")
        .trim()
        .to_string();

    (key, value)
}

pub fn get_listeners(
    servers: &Vec<Server>,
) -> AppResult<Vec<TcpListener>> {
    let mut mux_listeners = vec![];
    for server in servers {
        mux_listeners.push(server.listeners()?);
    }

    // Flattens all listeners.
    Ok(mux_listeners
        .into_iter()
        .flatten()
        .collect())
}

#[cfg(target_os = "macos")]
pub fn timeout(timeout_in_ms: u64) -> *const timespec {
    let duration = Duration::from_millis(timeout_in_ms);
    let secs = duration.as_secs() as time_t;
    let nanos = duration.subsec_nanos() as c_long;

    &timespec {
        tv_sec:  secs,
        tv_nsec: nanos,
    }
}
