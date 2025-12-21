use derive_more::{From, Into};
use derive_new::new;

/// A 2D point
#[derive(Copy, Clone, From, Into, new, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point {
    /// The x coordinate
    pub x: usize,
    /// The y coordinate
    pub y: usize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Point { x, y } = self;
        write!(f, "{x}x{y}")
    }
}
