use antbox_geom::Shape;

use crate::Color;
use crate::backend::Backend;

/// A [Shape] with a [Color]
#[derive(Copy, Clone, Debug)]
pub struct ShapeWithColor {
    shape: Shape,
    color: Color,
}

impl ShapeWithColor {
    pub(crate) fn render_to<B>(self, gfx: &mut B)
    where
        B: Backend,
    {
        gfx.render(self.shape, self.color);
    }
}

/// Convert into a [ShapeWithColor] given a [Color]
pub trait WithColor {
    /// Convert into a [ShapeWithColor] given a [Color]
    fn with_color(self, color: Color) -> ShapeWithColor;
}

impl<T> WithColor for T
where
    T: Into<Shape>,
{
    fn with_color(self, color: Color) -> ShapeWithColor {
        ShapeWithColor {
            shape: self.into(),
            color,
        }
    }
}
