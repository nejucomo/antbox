use antbox_geom::{Dimensions, Point, Rect};
use antbox_grid::{Bounds, GridCoord};

/// A [GridLayout] matches logical [Bounds] to a pixel view coordinates
#[derive(Copy, Clone, Debug)]
pub struct GridLayout {
    /// The logical bounds
    pub bounds: Bounds,
    /// The view size in (abstract) pixels
    pub view_size: Dimensions,
    /// The cell bounds around the origin in (abstract) pixels
    pub cell_dims: Dimensions,
}

impl GridLayout {
    /// Construct a new [Self]
    pub fn new(bounds: Bounds, view_size: Dimensions) -> Self {
        let cell_dims = view_size / (bounds.width, bounds.height);

        Self {
            bounds,
            view_size,
            cell_dims,
        }
    }

    /// Iterate over logical [BoundPoint]s and their associated pixel [Rect]s
    pub fn iter_pts_and_rects(&self) -> impl Iterator<Item = (GridCoord, Rect)> {
        let rect_top_left = Rect::from_point_and_dimensions(Point::ORIGIN, self.cell_dims);

        self.bounds.iter_points().map(move |coord| {
            (
                coord,
                rect_top_left.translate(self.cell_dims * (coord.x(), coord.y())),
            )
        })
    }
}

#[test]
fn verify_pts_and_rects() {
    let bounds = Bounds::new(2, 2);
    let gl = GridLayout::new(bounds, Dimensions::new(24., 18.));
    let bprs: Vec<(GridCoord, Rect)> = gl.iter_pts_and_rects().collect();
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
