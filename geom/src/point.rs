use derive_more::{From, Into};
use derive_new::new;

/// A 2D point
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct Point {
    /// The x coordinate
    pub x: usize,
    /// The y coordinate
    pub y: usize,
}
