use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;

/// An extension trait for [Rect]
pub trait RectExt {
    /// The center point of a [Rect]
    fn center(&self) -> Vec2;
}

impl RectExt for Rect {
    fn center(&self) -> Vec2 {
        let diag = self.bottom_right() - self.top_left();
        self.bottom_right() + (diag / 2.0)
    }
}
