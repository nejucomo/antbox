//! A very simple sleep-injecting non-realtime framerate timer
#![deny(unsafe_code, missing_docs)]

mod instantext;
mod ratelimit;
mod targetdelta;
mod timer;

pub use self::instantext::InstantExt;
pub use self::ratelimit::RateLimiter;
pub use self::targetdelta::TargetDelta;
pub use self::timer::TickTimer;
