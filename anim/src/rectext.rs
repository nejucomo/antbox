use extension_traits::extension;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;

/// An extension trait for [Rect]
#[extension(pub trait RectExt)]
impl Rect {
    /// The center of the [Rect]
    fn center(&self) -> Vec2 {
        self.top_left() + (self.diagonal() / 2.0)
    }

    /// The diagonal of the [Rect]
    fn diagonal(&self) -> Vec2 {
        self.bottom_right() - self.top_left()
    }

    /// The cell radius for the [Rect]
    fn cell_radius(&self) -> f32 {
        self.width().min(self.height()) * 0.5
    }
}

#[test]
fn center() {
    let r = Rect::from_tuples((10., 10.), (13., 14.));
    assert_eq!(r.center(), Vec2::new(11.5, 12.0));
}

#[test]
fn diagonal() {
    let r = Rect::from_tuples((10., 10.), (13., 14.));
    assert_eq!(r.diagonal(), Vec2::new(3., 4.));
    assert_eq!(r.diagonal() / 2.0, Vec2::new(1.5, 2.));
}

#[test]
fn cell_radius() {
    let r = Rect::from_tuples((10., 10.), (13., 14.));
    assert_eq!(r.cell_radius(), 1.5);
}
