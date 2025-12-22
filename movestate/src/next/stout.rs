use derive_more::{From, Into};
use derive_new::new;

/// A next `state` and `output`
#[derive(Debug, From, Into, new)]
pub struct Stout<S, O> {
    /// The new state
    pub state: S,
    /// The output
    pub output: O,
}

impl<S> From<S> for Stout<S, ()> {
    fn from(state: S) -> Self {
        Stout { state, output: () }
    }
}
