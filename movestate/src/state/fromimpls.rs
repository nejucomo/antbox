use crate::state::State;
use crate::stout::Stout;

impl<S> From<Stout<S, ()>> for State<S> {
    fn from(Stout { state, output: _ }: Stout<S, ()>) -> Self {
        Self::new(state)
    }
}
