use crate::{RenderScheduler, Renderable};

/// Similar to [Renderable] except which accepts an argument `A`
pub trait RenderWithArg<A>: Sized {
    /// Render `self` to `sch`, given argument `A`
    fn schedule_with_arg(self, sch: &mut RenderScheduler, arg: A);

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
    fn schedule(self, sch: &mut RenderScheduler) {
        let Self(r, a) = self;
        r.schedule_with_arg(sch, a);
    }
}

impl<Arg, A, B> RenderWithArg<Arg> for (A, B)
where
    Arg: Clone,
    A: RenderWithArg<Arg>,
    B: RenderWithArg<Arg>,
{
    fn schedule_with_arg(self, sch: &mut RenderScheduler, arg: Arg) {
        let (a, b) = self;
        a.schedule_with_arg(sch, arg.clone());
        b.schedule_with_arg(sch, arg);
    }
}

impl<Arg, A, B, C> RenderWithArg<Arg> for (A, B, C)
where
    Arg: Clone,
    A: RenderWithArg<Arg>,
    B: RenderWithArg<Arg>,
    C: RenderWithArg<Arg>,
{
    fn schedule_with_arg(self, sch: &mut RenderScheduler, arg: Arg) {
        let (a, b, c) = self;
        (a, (b, c)).schedule_with_arg(sch, arg);
    }
}
