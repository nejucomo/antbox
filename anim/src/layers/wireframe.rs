use antbox_geom::{Dimensions, Distance, Point};
use antbox_grid::Bounds;
use antbox_render::{Backend, Colorable as _, RenderWithArg};

use crate::{GridLayout, colors};

/// The wireframe layer, showing the grid cell rectangles
#[derive(Debug)]
pub struct WireFrame;

impl RenderWithArg<GridLayout> for WireFrame {
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, layout: GridLayout) {
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

        let view_width = f32::from(view_width);
        let view_height = f32::from(view_height);

        for col in 0..columns {
            let x = f32::from(cell_width * col);
            rb.render(
                Point::new(x, 0.0)
                    .vector_to((x, view_height))
                    .with_width(Distance::ONE)
                    .with_color(colors::WIRE_FRAME),
            );
        }
        for row in 0..rows {
            let y = f32::from(cell_height * row);
            rb.render(
                Point::new(0.0, y)
                    .vector_to((view_width, y))
                    .with_width(Distance::ONE)
                    .with_color(colors::WIRE_FRAME),
            );
        }
    }
}
