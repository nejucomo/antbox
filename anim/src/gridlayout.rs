use antbox_geom::{BoundPoint, Bounds};
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;

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

    /// Iterate over logical [BoundPoint]s and their associated pixel [Rect]s
    pub fn iter_pts_and_rects(&self) -> impl Iterator<Item = (BoundPoint, Rect)> {
        let Vec2 { x: cellw, y: cellh } = self.cell_bounds;

        self.bounds.iter_points().map(move |pt| {
            let left = cellw * (pt.x() as f32);
            let top = cellh * (pt.y() as f32);
            let right = left + cellw;
            let bottom = top + cellh;

            (
                pt,
                Rect::new(Vec2::new(left, top), Vec2::new(right, bottom)),
            )
        })
    }
}

#[test]
fn verify_pts_and_rects() {
    let bounds = Bounds::new(2, 2);
    let gl = GridLayout::new(bounds, Vec2::new(24., 18.));
    let bprs: Vec<(BoundPoint, Rect)> = gl.iter_pts_and_rects().collect();
    assert_eq!(
        bprs,
        &[
            (
                bounds.bind((0, 0)).unwrap(),
                Rect::from_tuples((0., 0.), (12., 9.))
            ),
            (
                bounds.bind((1, 0)).unwrap(),
                Rect::from_tuples((12., 0.), (24., 9.))
            ),
            (
                bounds.bind((0, 1)).unwrap(),
                Rect::from_tuples((0., 9.), (12., 18.))
            ),
            (
                bounds.bind((1, 1)).unwrap(),
                Rect::from_tuples((12., 9.), (24., 18.))
            ),
        ]
    );
}
