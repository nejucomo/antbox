use crate::MapInPlace;

impl<T> MapInPlace<T> for Option<T> {
    fn unwrap_state(self) -> T {
        self.expect("`MoveSlot::unwrap_mip` on invalid `None` state.")
    }

    fn opt_state(self) -> Option<T> {
        self
    }

    fn mip_out<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(T) -> (T, O),
    {
        let (x, o) = f(self.take().unwrap());
        *self = Some(x);
        o
    }

    fn mip_out_opt<F, O>(&mut self, f: F) -> Option<O>
    where
        F: FnOnce(T) -> Option<(T, O)>,
    {
        self.take().and_then(f).map(state_applier(self))
    }

    fn mip_out_res<F, O, E>(&mut self, f: F) -> Result<O, E>
    where
        F: FnOnce(T) -> Result<(T, O), E>,
    {
        let s = self.take().unwrap_state();
        f(s).map(state_applier(self))
    }
}

fn state_applier<T, O>(opts: &mut Option<T>) -> impl FnOnce((T, O)) -> O {
    |(s, o)| {
        *opts = Some(s);
        o
    }
}
