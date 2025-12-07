use speedy2d::color::Color;

pub const BACKGROUND: Color = Color::from_rgb(0.12, 0.1, 0.17);
pub const WIRE_FRAME: Color = Color::from_rgb(0.2, 0.2, 0.25);

// pub const ANT: Color = Color::from_rgb(0.1, 0.1, 0.17);
//
pub const SEEDPOD: Color = Color::DARK_GRAY;
pub const SEED: Color = Color::from_rgb(0.96, 0.87, 0.7);
pub const FOOD: Color = Color::from_rgb(0.1, 0.5, 0.17);
pub const OVERCROWDED: Color = Color::from_rgb(0.153, 0.125, 0.042);

pub fn food_neighbor_count(c: u8) -> Color {
    assert!(c > 0 && c < 9, "{c:?}");
    let c = c as f32;
    if c <= 3.0 {
        SEED.interpolate(FOOD, c / 3.0)
    } else {
        FOOD.interpolate(OVERCROWDED, (c - 3.0) / 5.0)
    }
}

pub trait Interpolate {
    fn interpolate(self, other: Self, factor: f32) -> Self;
}

impl Interpolate for f32 {
    fn interpolate(self, other: f32, factor: f32) -> f32 {
        factor * (other - self) + self
    }
}

impl Interpolate for Color {
    fn interpolate(self, other: Color, factor: f32) -> Color {
        Color::from_rgba(
            self.r().interpolate(other.r(), factor),
            self.g().interpolate(other.g(), factor),
            self.b().interpolate(other.b(), factor),
            self.a().interpolate(other.a(), factor),
        )
    }
}
