use antbox_geom::Shape;

use crate::Color;

/// A simple abstract immediate-rendering backend
pub trait Backend {
    /// Clear the screen with the given `color`
    fn clear_screen(&mut self, color: Color);

    /// Render the given [Shape] with [Color]
    fn render(&mut self, shape: Shape, color: Color);
}

// impl DrawOnto for Line {
//     fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
//         let Line { vec, width } = self;

//         gfx.draw_line(vec.start.v2(), vec.to().v2(), width.into(), color);
//     }
// }

// impl DrawOnto for Circle {
//     fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
//         gfx.draw_circle(self.center.v2(), self.radius.into(), color);
//     }
// }

// impl DrawOnto for Rect {
//     fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
//         let d = self.diagonal();

//         gfx.draw_rectangle(
//             speedy2d::shape::Rectangle::new(d.start.v2(), d.to().v2()),
//             color,
//         );
//     }
// }

// impl DrawOnto for Shape {
//     fn draw_onto(self, gfx: &mut Graphics2D, color: Color) {
//         use Shape::*;

//         match self {
//             Circle(x) => x.draw_onto(gfx, color),
//             Line(x) => x.draw_onto(gfx, color),
//             Rect(x) => x.draw_onto(gfx, color),
//         }
//     }
// }

// trait IntoVec2 {
//     fn v2(self) -> Vec2;
// }

// impl IntoVec2 for Point {
//     fn v2(self) -> Vec2 {
//         let Point { x, y } = self;
//         Vec2 { x, y }
//     }
// }
