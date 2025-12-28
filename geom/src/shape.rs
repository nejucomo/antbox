use derive_more::From;

use crate::{Circle, Line};

/// A drawable [Shape]
#[derive(Copy, Clone, Debug, From)]
pub enum Shape {
    #[allow(missing_docs)]
    Circle(Circle),
    #[allow(missing_docs)]
    Line(Line),
}
