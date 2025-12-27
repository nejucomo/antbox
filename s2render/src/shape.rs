use derive_more::From;
use speedy2d::Graphics2D;
use speedy2d::color::Color;

use crate::Circle;
use crate::drawonto::DrawOnto;

/// A drawable [Shape]
#[derive(Copy, Clone, Debug, From)]
pub enum Shape {
    #[allow(missing_docs)]
    Circle(Circle),
}

impl DrawOnto for Shape {
    fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
        use Shape::*;

        match self {
            Circle(x) => x.draw_onto(gfx, color),
        }
    }
}
