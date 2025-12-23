use crate::starg::Starg;

/// [Option] extensions to codify [Slot](crate::Slot) invariants
pub(crate) trait OptionExt<T> {
    fn unslot(self) -> T;

    fn map_to_starg<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> Starg<T, O>;
}

impl<T> OptionExt<T> for Option<T> {
    fn unslot(self) -> T {
        self.expect("Slot invariant failed: no state present")
    }

    fn map_to_starg<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> Starg<T, O>,
    {
        let Starg { state, arg } = f(self.take().unslot());
        *self = Some(state);
        arg
    }
}
