use crate::{RenderCycle, Renderable};

/// Similar to [Renderable] except which accepts an argument `A`
pub trait RenderWithArg<A>: Sized {
    /// Render `self` to `sch`, given argument `A`
    fn schedule_with_arg(self, cycle: &mut RenderCycle, arg: A);

    /// Store `self` and `arg` into a [Renderable]
    fn with_render_arg(self, arg: A) -> RenderableWithArg<Self, A> {
        RenderableWithArg(self, arg)
    }
}

pub struct RenderableWithArg<R, A>(R, A);

impl<R, A> Renderable for RenderableWithArg<R, A>
where
    R: RenderWithArg<A>,
{
    fn schedule(self, cycle: &mut RenderCycle) {
        let Self(r, a) = self;
        r.schedule_with_arg(cycle, a);
    }
}

impl<Arg, A, B> RenderWithArg<Arg> for (A, B)
where
    Arg: Clone,
    A: RenderWithArg<Arg>,
    B: RenderWithArg<Arg>,
{
    fn schedule_with_arg(self, cycle: &mut RenderCycle, arg: Arg) {
        let (a, b) = self;
        a.schedule_with_arg(cycle, arg.clone());
        b.schedule_with_arg(cycle, arg);
    }
}

impl<Arg, A, B, C> RenderWithArg<Arg> for (A, B, C)
where
    Arg: Clone,
    A: RenderWithArg<Arg>,
    B: RenderWithArg<Arg>,
    C: RenderWithArg<Arg>,
{
    fn schedule_with_arg(self, cycle: &mut RenderCycle, arg: Arg) {
        let (a, b, c) = self;
        (a, (b, c)).schedule_with_arg(cycle, arg);
    }
}
