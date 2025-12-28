use derive_new::new;

use crate::Point;

/// A renderable [Circle]
#[derive(Copy, Clone, Debug, new)]
pub struct Circle {
    /// The `center` [Point] of the [Circle]
    pub center: Point,
    /// The `radius` of the [Circle]
    pub radius: f32,
}

impl Circle {
    /// Scale the radius
    pub fn scale(self, radf: f32) -> Self {
        Circle {
            radius: self.radius * radf,
            ..self
        }
    }
}
