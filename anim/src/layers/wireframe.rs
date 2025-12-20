use antbox_geom::Bounds;

use crate::{Drawable, GridLayout, colors};

/// The wireframe layer, showing the grid cell rectangles
#[derive(Debug)]
pub struct WireFrame;

impl Drawable<GridLayout> for WireFrame {
    fn draw_on(self, gfx: &mut speedy2d::Graphics2D, layout: GridLayout) {
        let GridLayout {
            bounds: Bounds { width, height },
            view_size,
            cell_bounds,
            cell_radius: _,
        } = layout;

        for col in 0..width {
            let x = (col as f32) * cell_bounds.x;
            gfx.draw_line((x, 0.0), (x, view_size.y), 1.0, colors::WIRE_FRAME);
        }
        for row in 0..height {
            let y = (row as f32) * cell_bounds.y;
            gfx.draw_line((0.0, y), (view_size.x, y), 1.0, colors::WIRE_FRAME);
        }
    }
}
