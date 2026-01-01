use antbox_color::Color;
use antbox_geom::Shape;

use crate::{Backend, Renderable};

/// A [Shape] and [Color] is a primitive [Backend] rendering unit
#[derive(Copy, Clone, Debug)]
pub struct ShapeAndColor {
    sh: Shape,
    clr: Color,
}

impl Renderable for ShapeAndColor {
    fn render_to<B: ?Sized + Backend>(self, rb: &mut B) {
        rb.render_shape_and_color(self.sh, self.clr)
    }
}

/// Any `Into<Shape>` is [Colorable]
pub trait Colorable: Into<Shape> {
    /// Combine with a [Color] into a [ShapeAndColor]
    fn with_color(self, color: Color) -> ShapeAndColor {
        ShapeAndColor {
            sh: self.into(),
            clr: color,
        }
    }
}

impl<T: Into<Shape>> Colorable for T {}
