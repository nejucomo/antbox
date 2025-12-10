use antbox_geom::Bounds;

use crate::{Drawable, GfxLayout, GridLayout, colors};

/// The wireframe layer, showing the grid cell rectangles
#[derive(Debug)]
pub struct WireFrame;

impl Drawable for WireFrame {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let GridLayout {
            bounds: Bounds { width, height },
            view_size,
            cell_bounds,
            cell_radius: _,
        } = g.grid_layout;

        for col in 0..width {
            let x = (col as f32) * cell_bounds.x;
            g.draw_line((x, 0.0), (x, view_size.y), 1.0, colors::WIRE_FRAME);
        }
        for row in 0..height {
            let y = (row as f32) * cell_bounds.y;
            g.draw_line((0.0, y), (view_size.x, y), 1.0, colors::WIRE_FRAME);
        }
    }
}
