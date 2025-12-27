use speedy2d::Graphics2D;
use speedy2d::color::Color;

use crate::Shape;
use crate::drawonto::DrawOnto;

/// A [Shape] with a [Color]
#[derive(Copy, Clone, Debug)]
pub struct ShapeWithColor {
    color: Color,
    shape: Shape,
}

impl ShapeWithColor {
    /// Immediately draw onto `gfx`
    pub fn draw_onto(self, gfx: &mut Graphics2D) {
        self.shape.draw_onto(gfx, self.color);
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
            color,
            shape: self.into(),
        }
    }
}
