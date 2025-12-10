use antbox_geom::{BoundPoint, Bounds};
use speedy2d::dimen::Vec2;

/// A [GridLayout] matches logical [Bounds] to a pixel view coordinates
#[derive(Copy, Clone, Debug)]
pub struct GridLayout {
    /// The logical bounds
    pub bounds: Bounds,
    /// The view size in (abstract) pixels
    pub view_size: Vec2,
    /// The cell size in (abstract) pixels
    pub cell_bounds: Vec2,
    /// The cell radius in (abstract) pixels
    pub cell_radius: f32,
}

impl GridLayout {
    /// Construct a new [Self]
    pub fn new(bounds: Bounds, view_size: Vec2) -> Self {
        let cell_bounds = {
            let w32 = bounds.width as f32;
            let h32 = bounds.height as f32;

            Vec2::new(view_size.x / w32, view_size.y / h32)
        };

        Self {
            bounds,
            view_size,
            cell_bounds,
            cell_radius: cell_bounds.x.min(cell_bounds.y) / 2.0,
        }
    }

    /// Iterate over logical [BoundPoint]s and their associated abstract pixel center points
    pub fn iter_pts_and_centers(&self) -> impl Iterator<Item = (BoundPoint, Vec2)> {
        let Vec2 { x: cellw, y: cellh } = self.cell_bounds;

        self.bounds.iter_points().map(move |pt| {
            (
                pt,
                Vec2::new(
                    cellw / 2.0 * (1 + 2 * pt.x()) as f32,
                    cellh / 2.0 * (1 + 2 * pt.y()) as f32,
                ),
            )
        })
    }
}
