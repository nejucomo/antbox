/// [Option] extensions to codify [Slot](crate::Slot) invariants
pub(crate) trait OptionExt<T> {
    fn unslot(self) -> T;

    fn mealy_map<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> (T, O);
}

impl<T> OptionExt<T> for Option<T> {
    fn unslot(self) -> T {
        self.expect("Slot invariant failed: no state present")
    }

    fn mealy_map<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> (T, O),
    {
        let (next, out) = f(self.take().unslot());
        *self = Some(next);
        out
    }
}
