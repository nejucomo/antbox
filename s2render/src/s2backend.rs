use antbox_color::Color;
use antbox_geom::{Circle, Line, Shape};
use antbox_render::Backend;
use derive_debug::Dbg;
use derive_more::From;
use speedy2d::Graphics2D;

use crate::into_s2::IntoS2 as _;

/// A newtype wrapper for `&mut` [Graphics2D] which implements [Backend]
#[derive(Dbg, From)]
pub struct Speedy2Backend<'a>(#[dbg(placeholder = "...")] &'a mut Graphics2D);

impl<'a> Backend for Speedy2Backend<'a> {
    fn clear_screen(&mut self, color: Color) {
        self.0.clear_screen(color.into_s2())
    }

    fn render_shape_and_color(&mut self, shape: Shape, color: Color) {
        let color = color.into_s2();

        match shape {
            Shape::Circle(Circle { center, radius }) => {
                self.0.draw_circle(center.into_s2(), radius.into(), color);
            }
            Shape::Line(Line { vec, width }) => {
                self.0
                    .draw_line(vec.start.into_s2(), vec.to().into_s2(), width.into(), color);
            }
            Shape::Rect(rect) => {
                self.0.draw_rectangle(rect.into_s2(), color);
            }
        }
    }
}
