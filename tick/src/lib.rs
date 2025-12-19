//! A very simple sleep-injecting non-realtime framerate timer
#![deny(unsafe_code, missing_docs)]

use std::time::{Duration, Instant};

use either::Either::{self, Left, Right};

/// The default frame rate for `antbox`
pub const DEFAULT_FRAME_RATE: f64 = 50.0;

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
    ///
    /// # Non-realtime Note
    ///
    /// If this is called after the target time, we reset the next target from now (so that the late/long duration tick delays all future ticks, unlike "real-time" frame-rate apps like games.
    pub fn sleep_check(&mut self) -> bool {
        let now = Instant::now();

        match diff_instants(now, self.target) {
            Right(earliness) => {
                std::thread::sleep(earliness);
                self.target += self.interval;
                true
            }
            Left(lateness) => {
                log::warn!("tick late: {lateness:?}");
                self.target = now + self.interval;
                false
            }
        }
    }
}

impl Default for TickTimer {
    fn default() -> Self {
        TickTimer::with_frame_rate(DEFAULT_FRAME_RATE)
    }
}

fn diff_instants(now: Instant, target: Instant) -> Either<Duration, Duration> {
    fn checked_duration_since(a: Instant, b: Instant) -> Option<Duration> {
        instant_to_none(a.checked_duration_since(b))
    }

    fn instant_to_none(optd: Option<Duration>) -> Option<Duration> {
        optd.and_then(|d| if d.is_zero() { None } else { Some(d) })
    }

    match (
        checked_duration_since(now, target),
        target.checked_duration_since(now),
    ) {
        (None, Some(r)) => Right(r),
        (Some(l), None) => Left(l),
        (l, r) => {
            panic!("Inconsistent diff_instants({now:?}, {target:?}): ({l:?}, {r:?}");
        }
    }
}
