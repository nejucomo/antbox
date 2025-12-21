use crate::{Ant, AntHole, OptInto, SteppedUpon};

/// TODO: Remove this
pub trait Objectish:
    Sized + Copy + OptInto<Ant> + OptInto<Ant> + OptInto<AntHole> + SteppedUpon
{
    /// Where or not `self` contains (or is) a `T`
    fn contains<T>(self) -> bool
    where
        Self: OptInto<T>,
    {
        let opt: Option<T> = self.opt_into();
        opt.is_some()
    }
}
