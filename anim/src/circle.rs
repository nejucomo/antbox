use derive_new::new;
use speedy2d::Graphics2D;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;

use crate::Drawable;

#[derive(Copy, Clone, Debug, new)]
pub struct Circle {
    center: Vec2,
    radius: f32,
}

impl Circle {
    pub fn scale(self, radf: f32) -> Self {
        Circle {
            radius: self.radius * radf,
            ..self
        }
    }
}

impl Drawable<Color> for Circle {
    fn draw_on(self, gfx: &mut Graphics2D, color: Color) {
        gfx.draw_circle(self.center, self.radius, color);
    }
}
