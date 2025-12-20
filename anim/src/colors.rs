use extension_traits::extension;
use speedy2d::color::Color;

/// The background color
pub(crate) const DIRT: Color = interpolate(
    Color::from_rgb(118. / 255., 80. / 255., 51.3 / 255.),
    Color::BLACK,
    0.5,
);

pub(crate) const WIRE_FRAME: Color = Color::from_rgb(0.1, 0.12, 0.16);

pub(crate) const SEEDPOD: Color = interpolate(Color::from_rgba(0.25, 0.25, 0.25, 0.75), DIRT, 0.5);
pub(crate) const SEED: Color = Color::from_rgb(0.96, 0.87, 0.7);
pub(crate) const RIPE: Color = Color::from_rgb(0.1, 0.5, 0.17);
pub(crate) const OVERCROWDED: Color = Color::from_rgb(0.153, 0.125, 0.042);
pub(crate) const FOOD_LIFE: Color = Color::from_rgb(0.5, 0.77, 0.46);

pub(crate) const ANT: Color = Color::from_rgb(100. / 255., 100. / 255., 170. / 255.);
pub(crate) const ANT_HOLE_ENTRANCE: Color = Color::BLACK;
pub(crate) const ANT_HOLE_IRIS: Color =
    interpolate(DIRT, interpolate(Color::RED, Color::WHITE, 0.3), 0.1);

pub(crate) fn food_neighbor_count(c: u8) -> Color {
    assert!(c < 9, "{c:?}");
    let c = c as f32;
    if c <= 3.0 {
        interpolate(SEED, RIPE, c / 3.0)
    } else {
        interpolate(RIPE, OVERCROWDED, (c - 3.0) / 5.0)
    }
}

pub const fn interpolate(from: Color, to: Color, factor: f32) -> Color {
    Color::from_rgba(
        interpolate_f32(from.r(), to.r(), factor),
        interpolate_f32(from.g(), to.g(), factor),
        interpolate_f32(from.b(), to.b(), factor),
        interpolate_f32(from.a(), to.a(), factor),
    )
}

const fn interpolate_f32(from: f32, to: f32, factor: f32) -> f32 {
    factor * (to - from) + from
}

#[extension(pub(crate) trait ColorExt)]
impl Color {
    fn with_alpha(self, a: f32) -> Color {
        Color::from_rgba(self.r(), self.g(), self.b(), a)
    }
}
