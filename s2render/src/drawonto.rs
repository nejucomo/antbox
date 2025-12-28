use antbox_geom::{Circle, Line, Shape};
use speedy2d::Graphics2D;
use speedy2d::color::Color;

pub(crate) trait DrawOnto {
    fn draw_onto(self, gfx: &mut Graphics2D, color: Color);
}

impl DrawOnto for Line {
    fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
        let Line {
            start,
            delta,
            width,
        } = self;
        let to = start + delta;

        gfx.draw_line(start.into(), to, width, color);
    }
}

impl DrawOnto for Circle {
    fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
        gfx.draw_circle(self.center, self.radius, color);
    }
}

impl DrawOnto for Shape {
    fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
        use Shape::*;

        match self {
            Circle(x) => x.draw_onto(gfx, color),
            Line(x) => x.draw_onto(gfx, color),
        }
    }
}
