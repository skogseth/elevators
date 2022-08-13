use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
pub struct Timer {
    now: Instant,
    duration: Duration,
}

impl Timer {
    pub fn from_secs(secs: u64) -> Timer {
        Timer {
            now: Instant::now(),
            duration: Duration::from_secs(secs),
        }
    }

    pub fn is_done(&self) -> bool {
        self.now.elapsed() >= self.duration
    }
}
