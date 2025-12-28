use antbox_grid::Bounds;
use antbox_s2render::{RenderCycle, RenderWithArg, Vec2Ext as _, WithColor as _};
use speedy2d::dimen::Vec2;

use crate::layers::Layer;
use crate::{GridLayout, colors};

/// The wireframe layer, showing the grid cell rectangles
#[derive(Debug)]
pub struct WireFrame;

impl RenderWithArg<GridLayout> for WireFrame {
    fn schedule_with_arg(self, cycle: &mut RenderCycle, layout: GridLayout) {
        let layer = Layer::WireFrame.scheduler(cycle);
        let GridLayout {
            bounds: Bounds { width, height },
            view_size,
            cell_bounds,
            cell_radius: _,
        } = layout;

        for col in 0..width {
            let x = (col as f32) * cell_bounds.x;
            layer.schedule(
                Vec2::new(x, 0.0)
                    .to((x, view_size.y), 1.0)
                    .with_color(colors::WIRE_FRAME),
            );
        }
        for row in 0..height {
            let y = (row as f32) * cell_bounds.y;
            layer.schedule(
                Vec2::new(0.0, y)
                    .to((view_size.x, y), 1.0)
                    .with_color(colors::WIRE_FRAME),
            );
        }
    }
}
