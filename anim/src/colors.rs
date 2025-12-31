use antbox_color::{BLACK, Color, rgb, rgba};

/// The background color
pub(crate) const DIRT: Color = rgb(118, 80, 51).interpolate(BLACK, 0.6);

pub(crate) const WIRE_FRAME: Color = rgb(26, 31, 41);

pub(crate) const SEEDPOD: Color = rgba(64, 64, 64, 191).interpolate(DIRT, 0.5);
pub(crate) const SEED: Color = rgb(245, 222, 179);
pub(crate) const RIPE: Color = rgb(26, 128, 43);
pub(crate) const OVERCROWDED: Color = rgb(39, 32, 11);
pub(crate) const FOOD_LIFE: Color = rgb(128, 196, 117);

pub(crate) const ANT: Color = rgb(100, 100, 170);
pub(crate) const ANT_HOLE_ENTRANCE: Color = Color::BLACK;
pub(crate) const ANT_HOLE_IRIS: Color =
    DIRT.interpolate(Color::RED.interpolate(Color::WHITE, 0.3), 0.1);

pub(crate) fn food_neighbor_count(c: u8) -> Color {
    assert!(c < 9, "{c:?}");
    let c = c as f32;
    if c <= 3.0 {
        SEED.interpolate(RIPE, c / 3.0)
    } else {
        RIPE.interpolate(OVERCROWDED, (c - 3.0) / 5.0)
    }
}

// #[extension(pub(crate) trait ColorExt)]
// impl Color {

//     fn with_alpha(self, a: f32) -> Color {
//         rgba(self.r(), self.g(), self.b(), a)
//     }
// }
