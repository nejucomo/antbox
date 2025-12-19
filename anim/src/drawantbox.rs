use antbox_state::{Ant, AntHole, Food, Object, Spot, State as AntboxState};
use speedy2d::color::Color;
use speedy2d::shape::Rect;

use crate::colors::Interpolate as _;
use crate::{Drawable, GfxLayout, RectExt as _, colors};

impl Drawable for &AntboxState {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let gl = g.grid_layout;
        for (pt, rect) in gl.iter_pts_and_rects() {
            (self[pt], rect).draw_on(g);
        }
    }
}

impl Drawable for (Spot, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let (spot, rect) = self;
        spot.object().map(|obj| (obj, rect)).draw_on(g);
    }
}

impl Drawable for (Object, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        use Object::*;

        let (obj, rect) = self;
        match obj {
            Food(x) => (x, rect).draw_on(g),
            Ant(x) => (x, rect).draw_on(g),
            AntHole(x) => (x, rect).draw_on(g),
        }
    }
}

impl Drawable for (Food, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let (_, rect) = self;
        let rad = g.grid_layout.cell_radius * 0.7;
        g.draw_circle(
            rect.center(),
            rad,
            colors::SEEDPOD.interpolate(Color::from_rgba(1., 1., 1., 0.), 0.6),
        );
    }
}

impl Drawable for (Ant, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        // TODO: head, throax, abdomen, food pellet
        let (_, rect) = self;
        let rad = g.grid_layout.cell_radius * 0.5;
        g.draw_circle(rect.center(), rad, colors::ANT);
    }
}

impl Drawable for (AntHole, Rect) {
    fn draw_on(self, g: &mut GfxLayout<'_>) {
        let (_, rect) = self;
        let c = rect.center();

        // Slightly too big to fit:
        let rad = g.grid_layout.cell_radius * 1.1;

        let colrads = [(colors::ANT_HOLE_IRIS, rad), (colors::ANT_HOLE, rad * 0.4)];

        for (color, rad) in colrads {
            g.draw_circle(c, rad, color);
        }
    }
}
