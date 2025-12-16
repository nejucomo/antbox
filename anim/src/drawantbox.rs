use antbox_clife::ConwayGrid as _;
use antbox_state::State as AntboxState;

use crate::{Drawable, RectExt as _, colors};

impl Drawable for &AntboxState {
    fn draw_on(self, g: &mut crate::GfxLayout<'_>) {
        let gl = g.grid_layout;
        let rad = gl.cell_radius * 0.9;
        for (pt, rect) in gl.iter_pts_and_rects() {
            let (life, _) = self.life_and_neighbors(pt);
            if life {
                g.draw_circle(rect.center(), rad, colors::SEEDPOD);
            }
        }
    }
}
