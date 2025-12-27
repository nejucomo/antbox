use speedy2d::Graphics2D;
use speedy2d::color::Color;

use crate::drawonto::DrawOnto;
use crate::{Circle, Element, Layer, Shape};

/// A [Shape] with a [Color]
#[derive(Copy, Clone, Debug)]
pub struct ShapeWithColor {
    color: Color,
    shape: Shape,
}

impl ShapeWithColor {
    /// Convert to an [Element] by specifying the target [Layer]
    pub fn on_layer(self, layer: Layer) -> Element {
        Element::new(layer, self)
    }

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

impl WithColor for Shape {
    fn with_color(self, color: Color) -> ShapeWithColor {
        ShapeWithColor { color, shape: self }
    }
}

impl WithColor for Circle {
    fn with_color(self, color: Color) -> ShapeWithColor {
        Shape::from(self).with_color(color)
    }
}
