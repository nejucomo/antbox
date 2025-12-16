use speedy2d::color::Color;

/// The background color
pub(crate) const BACKGROUND: Color = Color::from_rgb(0.12, 0.1, 0.17);
pub(crate) const WIRE_FRAME: Color = Color::from_rgb(0.1, 0.12, 0.16);

// pub(crate) const ANT: Color = Color::from_rgb(0.1, 0.1, 0.17);

pub(crate) const SEEDPOD: Color = Color::from_rgba(0.25, 0.25, 0.25, 0.75);
pub(crate) const SEED: Color = Color::from_rgb(0.96, 0.87, 0.7);
pub(crate) const RIPE: Color = Color::from_rgb(0.1, 0.5, 0.17);
pub(crate) const OVERCROWDED: Color = Color::from_rgb(0.153, 0.125, 0.042);
pub(crate) const FOOD_LIFE: Color = Color::from_rgb(0.5, 0.77, 0.46);

pub(crate) fn food_neighbor_count(c: u8) -> Color {
    assert!(c < 9, "{c:?}");
    let c = c as f32;
    if c <= 3.0 {
        SEED.interpolate(RIPE, c / 3.0)
    } else {
        RIPE.interpolate(OVERCROWDED, (c - 3.0) / 5.0)
    }
}

pub(crate) trait Interpolate {
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
