#[cfg(target_os = "macos")]
use libc::{
    c_long,
    time_t,
    timespec,
};
use {
    super::{AppErr, AppResult},
    crate::message::{Method, Resource},
    rand::{distributions::Alphanumeric, Rng},
    std::{
        fs::{self, File},
        io::{BufRead, BufReader, ErrorKind, Write},
        net::TcpStream,
        time::{SystemTime, UNIX_EPOCH},
    },
};
use crate::{utils::SESSION_STORE, Request};
#[cfg(target_os = "macos")]
use {
    libc::{c_long, time_t, timespec},
    std::time::Duration,
};

#[cfg(target_os = "macos")]
pub fn timeout(timeout_in_ms: u64) -> *const timespec {
    let duration = Duration::from_millis(timeout_in_ms);
    let secs = duration.as_secs() as time_t;
    let nanos = duration.subsec_nanos() as c_long;

    &timespec {
        tv_sec: secs,
        tv_nsec: nanos,
    }
}

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

pub fn generate_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn get_session_id(cookie: &str) -> Option<String> {
    cookie
        .split(';')
        .find(|s| {
            s.trim()
                .starts_with("session_id=")
        })
        .map(|s| s.trim()["session_id=".len()..].to_string())
}

pub fn has_valid_session(req: &Request) -> bool {
    if let Some(cookie) = req.headers.get("Cookie") {
        if let Some(session_id) = get_session_id(cookie) {
            return SESSION_STORE.validate_session(&session_id);
        }
    }
    false
}

pub fn read_buffer(stream: &TcpStream) -> AppResult<String> {
    let mut buf_reader = BufReader::new(stream);
    let mut req_str = String::new();

    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                req_str.push_str(&line);

                if line == "\r\n" || line == "\n" {
                    break;
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }
    }

    if req_str.is_empty() {
        Err(AppErr::EmptyBuffer)
    } else {
        Ok(req_str)
    }
}
pub const SESSION_FILE: &str = "sessions.txt";
pub const CLEANUP_FILE: &str = "last_cleanup.txt";

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn init_files(timestamp: u64) -> Result<(), String> {
    if !std::path::Path::new(SESSION_FILE).exists() {
        File::create(SESSION_FILE).map_err(|e| e.to_string())?;
    }
    if !std::path::Path::new(CLEANUP_FILE).exists() {
        let mut file =
            File::create(CLEANUP_FILE).map_err(|e| e.to_string())?;
        writeln!(file, "{}", timestamp).map_err(|e| {
            format!(
                "Impossible d'écrire le timestamp initial: {}",
                e
            )
        })?;
    }
    Ok(())
}

pub fn read_sessions() -> Result<Vec<String>, String> {
    let file = File::open(SESSION_FILE).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(Result::ok)
        .collect())
}

pub fn write_sessions(sessions: &[String]) -> Result<(), String> {
    let mut file =
        File::create(SESSION_FILE).map_err(|e| e.to_string())?;
    for session in sessions {
        writeln!(file, "{}", session)
            .map_err(|_| "Impossible d'écrire la session")?;
    }
    Ok(())
}

pub fn get_last_cleanup() -> u64 {
    fs::read_to_string(CLEANUP_FILE)
        .ok()
        .and_then(|content| content.trim().parse().ok())
        .unwrap_or(0)
}

pub fn update_last_cleanup(timestamp: u64) -> Result<(), String> {
    File::create(CLEANUP_FILE)
        .map_err(|e| e.to_string())?
        .write_all(
            timestamp
                .to_string()
                .as_bytes(),
        )
        .map_err(|e| e.to_string())
}
