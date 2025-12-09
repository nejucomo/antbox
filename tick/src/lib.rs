//! A very simple sleep-injecting non-realtime framerate timer
#![deny(unsafe_code, missing_docs)]

use std::time::{Duration, Instant};

/// The default frame rate for `antbox`
pub const DEFAULT_FRAME_RATE: f64 = 10.0;

/// Track the next target instant to achieve a given framerate
#[derive(Copy, Clone, Debug)]
pub struct TickTimer {
    interval: Duration,
    next: Instant,
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
            next: Instant::now(),
        }
    }

    /// Sleep if necessary to achieve the target framerate, then update next target time
    ///
    /// # Return
    ///
    /// This returns whether the app is on schedule (e.g. the call began before the target time and slept after the target time without sleep).
    ///
    /// # Non-realtime Note
    ///
    /// If this is called after the target time, we reset the next target from now (so that the late/long duration tick delays all future ticks, unlike "real-time" frame-rate apps like games.
    pub fn sleep_check(&mut self) -> bool {
        let now = Instant::now();

        let earliness = self.next.checked_duration_since(now);
        let lateness = now.checked_duration_since(self.next);

        let on_time = match (earliness, lateness) {
            (Some(earliness), None) => {
                std::thread::sleep(earliness);
                true
            }
            (None, Some(lateness)) => {
                log::debug!("tick late: {lateness:?}");
                false
            }
            (earliness, lateness) => {
                panic!(
                    "Inconsistent early/late calculation: early {earliness:?}, lateness: {lateness:?} in {self:#?}"
                );
            }
        };
        self.next = now + self.interval;
        on_time
    }
}

impl Default for TickTimer {
    fn default() -> Self {
        TickTimer::with_frame_rate(DEFAULT_FRAME_RATE)
    }
}

// pub fn launch(d: Duration, ues: UserEventSender<Tick>) {
//     spawn(move || {
//         let mut next = Instant::now();

//         loop {
//             sleep(next - Instant::now());
//             let now = Instant::now();
//             ues.send_event(Tick(now));
//             next = now + d;
//         }
//     });
// }
