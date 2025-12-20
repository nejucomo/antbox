use std::time::{Duration, Instant};

use crate::TargetDelta::Early;
use crate::{InstantExt as _, TargetDelta};

/// Track the next target instant to achieve a given framerate
#[derive(Copy, Clone, Debug)]
pub struct TickTimer {
    interval: Duration,
    target: Instant,
}

impl TickTimer {
    /// Construct a [TickTimer] with the given framerate frequency
    pub fn with_frame_rate(freq: f64) -> Self {
        TickTimer::with_interval(Duration::from_millis((1000.0 / freq) as u64))
    }

    /// Construct a [TickTimer] with the interval (inverse frequency)
    pub fn with_interval(interval: Duration) -> Self {
        TickTimer {
            interval,
            target: Instant::now(),
        }
    }

    /// Sleep if necessary to achieve the target framerate, then update next target time
    ///
    /// # Return
    ///
    /// This returns whether the app is on schedule (e.g. the call began before the target time and slept after the target time without sleep).
    pub fn sleep_update(&mut self) {
        if let Early(earliness) = self.delta_update() {
            std::thread::sleep(earliness);
            self.target += self.interval;
        }
    }

    /// Similar to [Self::target_delta] except if we're [Late](TargetDelta::Late), advance our target to the next future interval target
    pub fn delta_update(&mut self) -> TargetDelta {
        let td = self.target_delta();
        while self.target_delta().is_late() {
            self.target += self.interval;
        }
        td
    }

    /// The delta from now to the next target
    pub fn target_delta(&self) -> TargetDelta {
        self.target.delta_from_now()
    }
}
