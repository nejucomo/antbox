use derive_more::{From, Into};
use derive_new::new;

#[derive(Debug, From, Into, new)]
pub struct Bounds<T> {
    pub width: T,
    pub height: T,
}
