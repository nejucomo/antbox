use crate::{Drawable, GfxLayout, colors};

/// The background layer
#[derive(Debug)]
pub struct Background;

impl Drawable for Background {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        g.clear_screen(colors::DIRT);
    }
}
