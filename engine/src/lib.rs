//! The `antbox` engine encapsulates all state evolution in a standalone thread
#![deny(unsafe_code, missing_docs)]

mod notification;
mod notifier;
mod spawn;
mod ticktimer;

pub use self::notification::Notification;
pub use self::notifier::Notifier;
pub use self::spawn::spawn;
pub use self::ticktimer::TickTimer;
