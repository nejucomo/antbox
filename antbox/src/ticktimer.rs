use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct TickTimer {
    interval: Duration,
    next: Instant,
}

impl TickTimer {
    pub fn new(millis: u64) -> Self {
        let interval = Duration::from_millis(millis);
        TickTimer {
            interval,
            next: Instant::now() + interval,
        }
    }

    pub fn wait_for_tick(&mut self) {
        std::thread::sleep(self.next - Instant::now());
        self.next = Instant::now() + self.interval;
        log::debug!("TICK");
    }
}

impl Default for TickTimer {
    fn default() -> Self {
        Self::new(2000)
    }
}
