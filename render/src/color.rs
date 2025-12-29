use crate::{RenderCycle, Renderable};

/// An `RGBA` color representation with bands in [0, 1]
#[derive(Copy, Clone, Debug)]
pub struct Color {
    #[allow(missing_docs)]
    pub r: f32,
    #[allow(missing_docs)]
    pub g: f32,
    #[allow(missing_docs)]
    pub b: f32,
    #[allow(missing_docs)]
    pub a: f32,
}

impl u8 {
    const fn to_norm(self) -> f32 {
        (self as f32) / (u8::MAX as f32)
    }
}

impl Color {
    pub const fn u8_to_norm(u: u8) -> f32 {
        u as f32 / u8::max as f32
    }

    pub const fn from_f_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {}

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b,
            a: u8::MAX,
        }
    }
}

impl Renderable for Color {
    fn schedule(self, cycle: &mut RenderCycle) {
        cycle.schedule_bg_color(self);
    }
}
