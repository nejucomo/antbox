/// [Option] extensions to codify [Slot](crate::Slot) invariants
pub(crate) trait OptionExt<T> {
    fn unslot(self) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    fn unslot(self) -> T {
        self.expect("Slot invariant failed: no state present")
    }
}
