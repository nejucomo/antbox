use speedy2d::Graphics2D;
use speedy2d::color::Color;

pub(crate) trait DrawOnto {
    fn draw_onto(self, gfx: &mut Graphics2D, color: Color);
}
