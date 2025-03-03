use {
    crate::utils::globals::CLEANUP_FILE,
    std::{
        fs::{
            read_to_string,
            File,
        },
        io::Write,
        time::{
            SystemTime,
            UNIX_EPOCH,
        },
    },
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

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn get_last_cleanup() -> u64 {
    read_to_string(CLEANUP_FILE)
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
