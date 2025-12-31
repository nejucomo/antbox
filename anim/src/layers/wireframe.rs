use antbox_geom::{Dimensions, Point};
use antbox_grid::Bounds;
use antbox_render::{Backend, RenderWithArg};

use crate::{GridLayout, colors};

/// The wireframe layer, showing the grid cell rectangles
#[derive(Debug)]
pub struct WireFrame;

impl RenderWithArg<GridLayout> for WireFrame {
    fn render_with_arg<B: Backend>(self, rb: &mut B, layout: GridLayout) {
        let GridLayout {
            bounds:
                Bounds {
                    width: columns,
                    height: rows,
                },
            view_size:
                Dimensions {
                    width: view_width,
                    height: view_height,
                },
            cell_dims:
                Dimensions {
                    width: cell_width,
                    height: cell_height,
                },
        } = layout;

        for col in 0..columns {
            let x = cell_width * col;
            rb.render(
                Point::new(x, 0.0).vector_to((x, view_height)),
                colors::WIRE_FRAME,
            );
        }
        for row in 0..rows {
            let y = cell_height * row;
            rb.render(
                Point::new(0.0, y).vector_to((view_width, y)),
                colors::WIRE_FRAME,
            );
        }
    }
}
