use crate::gfxlayout::GfxLayout;

/// An object which can be drawn onto a [GfxLayout]
pub trait Drawable {
    /// Draw `self` onto a [GfxLayout]
    fn draw_on(self, g: &mut GfxLayout<'_>);
}
