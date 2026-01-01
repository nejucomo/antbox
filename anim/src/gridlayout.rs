use std::num::NonZero;

use antbox_geom::{Dimensions, Rect, Transformable as _};
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
        let cell_dims = view_size
            / (
                NonZero::new(bounds.width).unwrap(),
                NonZero::new(bounds.height).unwrap(),
            );

        Self {
            bounds,
            view_size,
            cell_dims,
        }
    }

    /// Iterate over logical [BoundPoint]s and their associated pixel [Rect]s
    pub fn iter_pts_and_rects(&self) -> impl Iterator<Item = (GridCoord, Rect)> {
        let rect_top_left = Rect::from_origin_with_dimensions(self.cell_dims);

        self.bounds.iter_points().map(move |coord| {
            // The dimensions of all space above and to the left of this cell's [Rect]:
            let upper_left_quadrant = self.cell_dims * (coord.x(), coord.y());

            (
                coord,
                rect_top_left.translate(upper_left_quadrant.into_bottom_right()),
            )
        })
    }
}

#[test]
fn verify_pts_and_rects() {
    use antbox_geom::{Distance, Point};

    let bounds = Bounds::new(3, 2);

    let gl = GridLayout::new(
        bounds,
        Dimensions::new(Distance::fromp_f32(24.0), Distance::fromp_f32(18.0)),
    );

    let bprs: Vec<(GridCoord, Rect)> = gl.iter_pts_and_rects().collect();

    fn rect(x1: f32, y1: f32, x2: f32, y2: f32) -> Rect {
        Rect::from_diagonal(Point::new(x1, y1).vector_to((x2, y2)))
    }

    assert_eq!(
        bprs,
        &[
            (bounds.bind((0, 0)).unwrap(), rect(0., 0., 12., 9.)),
            (bounds.bind((1, 0)).unwrap(), rect(8., 0., 24., 9.)),
            (bounds.bind((2, 0)).unwrap(), rect(16., 0., 24., 9.)),
        ]
    );
}
