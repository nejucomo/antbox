use extension_traits::extension;
use speedy2d::dimen::Vec2;

use crate::Line;
use crate::circle::Circle;

/// Extensions to [Vec2]
#[extension(pub trait Vec2Ext)]
impl Vec2 {
    /// The circle centered at `self` with radius `r`
    fn with_radius(self, r: f32) -> Circle {
        Circle::new(self, r)
    }

    /// A line from `self` to `to` with width `width`
    fn to<V>(self, to: V, width: f32) -> Line
    where
        V: Into<Vec2>,
    {
        Line::new(self, to, width)
    }
}
