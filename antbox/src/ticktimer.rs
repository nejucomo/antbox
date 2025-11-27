use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct TickTimer {
    interval: Duration,
    previnst: Option<Instant>,
}

impl TickTimer {
    pub fn new(millis: u64) -> Self {
        TickTimer {
            interval: Duration::from_millis(millis),
            previnst: None,
        }
    }

    pub fn check_update(&mut self) -> bool {
        let now = Instant::now();
        let do_update = self
            .previnst
            .map(|pi| now >= pi + self.interval)
            .unwrap_or(true);

        if do_update {
            self.previnst = Some(now);
        }
        do_update
    }
}

impl Default for TickTimer {
    fn default() -> Self {
        Self::new(50)
    }
}
