use extension_traits::extension;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;

/// An extension trait for [Rect]
#[extension(pub trait RectExt)]
impl Rect {
    /// The center of the [Rect]
    fn center(&self) -> Vec2 {
        self.bottom_right() + (self.diagonal() / 2.0)
    }

    /// The diagonal of the [Rect]
    fn diagonal(&self) -> Vec2 {
        self.bottom_right() - self.top_left()
    }

    /// The cell radius for the [Rect]
    fn cell_radius(&self) -> f32 {
        self.width().min(self.height())
    }
}
