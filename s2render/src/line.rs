use derive_new::new;
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;

use crate::drawonto::DrawOnto;

/// A renderable [Line]
#[derive(Copy, Clone, Debug, new)]
pub struct Line {
    #[new(into)]
    from: Vec2,
    #[new(into)]
    to: Vec2,
    width: f32,
}

impl DrawOnto for Line {
    fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
        gfx.draw_line(self.from, self.to, self.width, color);
    }
}
