pub trait IntoS2 {
    type S2;

    fn into_s2(self) -> Self::S2;
}

impl IntoS2 for antbox_color::Color {
    type S2 = speedy2d::color::Color;

    fn into_s2(self) -> Self::S2 {
        speedy2d::color::Color::from_rgba(
            self.r.into(),
            self.g.into(),
            self.b.into(),
            self.a.into(),
        )
    }
}

impl IntoS2 for antbox_geom::Point {
    type S2 = speedy2d::dimen::Vec2;

    fn into_s2(self) -> Self::S2 {
        speedy2d::dimen::Vec2::new(self.x, self.y)
    }
}

impl IntoS2 for antbox_geom::Rect {
    type S2 = speedy2d::shape::Rect;

    fn into_s2(self) -> Self::S2 {
        speedy2d::shape::Rect::new(self.top_left().into_s2(), self.bottom_right().into_s2())
    }
}
