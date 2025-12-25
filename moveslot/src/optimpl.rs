use crate::MapInPlace;

impl<T> MapInPlace<T> for Option<T> {
    fn unwrap_mip(self) -> T {
        self.expect("`MoveSlot::unwrap_mip` on invalid `None` state.")
    }

    fn mapout_in_place<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> (T, O),
    {
        let (x, o) = f(self.take().unwrap());
        *self = Some(x);
        o
    }
}
