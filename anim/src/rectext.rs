use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;

/// An extension trait for [Rect]
pub trait RectExt {
    /// The center point of a [Rect]
    fn center(&self) -> Vec2;

    /// The diagonal from top-left to bottom-right
    fn diagonal(&self) -> Vec2;
}

impl RectExt for Rect {
    fn center(&self) -> Vec2 {
        self.bottom_right() + (self.diagonal() / 2.0)
    }

    fn diagonal(&self) -> Vec2 {
        self.bottom_right() - self.top_left()
    }
}
