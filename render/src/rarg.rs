use crate::{Backend, Renderable};

/// Similar to [Renderable] except which accepts an argument `A`
pub trait RenderWithArg<A>: Sized {
    /// Render `self` to `sch`, given argument `A`
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, arg: A);

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
    fn render_to<B: ?Sized + Backend>(self, rb: &mut B) {
        let Self(r, a) = self;
        r.render_with_arg(rb, a);
    }
}

impl<A, X, Y> RenderWithArg<A> for (X, Y)
where
    A: Clone,
    X: RenderWithArg<A>,
    Y: RenderWithArg<A>,
{
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, arg: A) {
        let (a, b) = self;
        a.render_with_arg(rb, arg.clone());
        b.render_with_arg(rb, arg);
    }
}

impl<A, X, Y, Z> RenderWithArg<A> for (X, Y, Z)
where
    A: Clone,
    X: RenderWithArg<A>,
    Y: RenderWithArg<A>,
    Z: RenderWithArg<A>,
{
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, arg: A) {
        let (a, b, c) = self;
        (a, (b, c)).render_with_arg(rb, arg);
    }
}
