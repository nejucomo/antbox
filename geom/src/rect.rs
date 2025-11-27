use derive_more::{From, Into};
use derive_new::new;

#[derive(Debug, From, Into, new)]
pub struct Rect<T> {
    pub top_left: Point<T>,
    pub bounds: Bounds<T>,
}
