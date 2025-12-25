use crate::state::State;
use crate::stout::Stout;

impl<S> From<State<S>> for Stout<S, ()> {
    fn from(State { state }: State<S>) -> Self {
        Self::new(state, ())
    }
}
