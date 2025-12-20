use speedy2d::Graphics2D;

/// An object which can be drawn onto a [Graphics2D]
pub trait Drawable<T> {
    /// Draw `self` onto `g`
    fn draw_on(self, gfx: &mut Graphics2D, arg: T);
}

impl<B, T> Drawable<T> for Option<B>
where
    B: Drawable<T>,
{
    fn draw_on(self, gfx: &mut Graphics2D, arg: T) {
        if let Some(v) = self {
            v.draw_on(gfx, arg);
        }
    }
}
