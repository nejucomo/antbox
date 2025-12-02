use std::time::{Duration, Instant};

/// A timer for controlling engine update frequency
#[derive(Debug)]
pub struct TickTimer {
    interval: Duration,
    next: Instant,
}

impl TickTimer {
    /// Start a new timer with a `millis` interval in ms
    pub fn start(millis: u64) -> Self {
        let interval = Duration::from_millis(millis);
        TickTimer {
            interval,
            next: Instant::now() + interval,
        }
    }

    pub(crate) fn wait_for_tick(&mut self) {
        std::thread::sleep(self.next - Instant::now());
        self.next = Instant::now() + self.interval;
        log::debug!("TICK");
    }
}
