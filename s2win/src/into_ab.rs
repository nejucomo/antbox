use antbox_geom::{Dimensions, Distance, Point};
use speedy2d::dimen::{UVec2, Vec2, Vector2};

pub(crate) trait IntoAntbox {
    type AB;

    fn into_antbox(self) -> Self::AB;
}

impl IntoAntbox for UVec2 {
    type AB = Dimensions;

    fn into_antbox(self) -> Self::AB {
        let Vector2 { x, y } = self.into_f32();
        let width = Distance::fromp_f32(x);
        let height = Distance::fromp_f32(y);
        Dimensions { width, height }
    }
}

impl IntoAntbox for Vec2 {
    type AB = Point;

    fn into_antbox(self) -> Self::AB {
        let Vector2 { x, y } = self;
        Point { x, y }
    }
}
