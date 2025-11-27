use derive_more::{From, Into};
use derive_new::new;

#[derive(Debug, From, Into, new)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
