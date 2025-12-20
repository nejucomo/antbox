use std::time::{Duration, Instant};

use derive_more::IsVariant;

use self::TargetDelta::{Early, Late};

/// A [Duration] ahead or past a target time
#[derive(Copy, Clone, Debug, IsVariant)]
pub enum TargetDelta {
    /// A [Duration] ahead of a target time
    Early(Duration),
    /// A [Duration] past of a target time
    Late(Duration),
}

impl TargetDelta {
    pub(crate) fn new(now: Instant, target: Instant) -> TargetDelta {
        match (
            checked_duration_since(now, target),
            target.checked_duration_since(now),
        ) {
            (None, Some(r)) => Early(r),
            (Some(l), None) => Late(l),
            (l, r) => {
                panic!("Inconsistent diff_instants({now:?}, {target:?}): ({l:?}, {r:?}");
            }
        }
    }
}

fn checked_duration_since(a: Instant, b: Instant) -> Option<Duration> {
    instant_to_none(a.checked_duration_since(b))
}

fn instant_to_none(optd: Option<Duration>) -> Option<Duration> {
    optd.and_then(|d| if d.is_zero() { None } else { Some(d) })
}
