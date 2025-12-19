use crate::gfxlayout::GfxLayout;

/// An object which can be drawn onto a [GfxLayout]
pub trait Drawable {
    /// Draw `self` onto a [GfxLayout]
    fn draw_on(self, g: &mut GfxLayout<'_>);
}

impl<T> Drawable for Option<T>
where
    T: Drawable,
{
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        if let Some(v) = self {
            v.draw_on(g);
        }
    }
}
