use antbox_geom::{Circle, Line, Point, Shape};
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;

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

        gfx.draw_line(start.v2(), to.v2(), width.into(), color);
    }
}

impl DrawOnto for Circle {
    fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
        gfx.draw_circle(self.center.v2(), self.radius.into(), color);
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

trait IntoVec2 {
    fn v2(self) -> Vec2;
}

impl IntoVec2 for Point {
    fn v2(self) -> Vec2 {
        let Point { x, y } = self;
        Vec2 { x, y }
    }
}
