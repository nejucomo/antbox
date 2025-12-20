use crate::{Drawable, colors};

/// The background layer
#[derive(Debug)]
pub struct Background;

impl Drawable<()> for Background {
    fn draw_on(self, g: &mut speedy2d::Graphics2D, (): ()) {
        g.clear_screen(colors::DIRT);
    }
}
