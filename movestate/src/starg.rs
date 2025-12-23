use derive_more::{From, Into};
use derive_new::new;

/// A `state` and associated `arg`
#[derive(Debug, From, Into, new)]
pub struct Starg<S, A> {
    /// The new state
    pub state: S,
    /// The arg
    pub arg: A,
}

impl<S> From<S> for Starg<S, ()> {
    fn from(state: S) -> Self {
        Starg { state, arg: () }
    }
}
