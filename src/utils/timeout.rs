use std::time::{Duration, Instant};

pub struct Timeout {
    start: Instant,
    duration: Duration,
}

impl Timeout {
    pub fn new(duration: Duration) -> Self {
        Timeout {
            start: Instant::now(),
            duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() - self.start >= self.duration
    }
}