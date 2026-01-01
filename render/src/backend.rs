use antbox_color::Color;
use antbox_geom::Shape;

/// A simple abstract immediate-rendering backend
pub trait Backend {
    /// Clear the screen with the given `color`
    fn clear_screen(&mut self, color: Color);

    /// Render `shape` with [Color]
    fn render<S: Into<Shape>>(&mut self, shape: S, color: Color) {
        self.render_shape(shape.into(), color)
    }

    /// Render the given [Shape] with [Color]
    fn render_shape(&mut self, shape: Shape, color: Color);
}
