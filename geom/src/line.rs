use derive_new::new;

use crate::Point;

/// A [Line] segment
#[derive(Copy, Clone, Debug, new)]
pub struct Line {
    /// The starting [Point]
    #[new(into)]
    pub start: Point,
    #[new(into)]
    /// The delta [Point]; the line ends at `self.start + self.delta`
    pub delta: Point,
    /// The width
    pub width: f32,
}
