use antbox_color::Color;
use antbox_geom::Shape;

/// A simple abstract immediate-rendering backend
pub trait Backend {
    /// Clear the screen with the given `color`
    fn clear_screen(&mut self, color: Color);

    /// Render the given [Shape] with [Color]
    fn render(&mut self, shape: Shape, color: Color);
}
