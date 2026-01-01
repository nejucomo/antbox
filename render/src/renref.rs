use crate::{Backend, RenderWithArg};

/// Similar to [RenderWithArg] with a `&self` receiver
pub trait RenderRefWithArg<A> {
    /// Render `self` to `sch`, given argument `A`
    fn render_ref_with_arg<B: ?Sized + Backend>(&self, rb: &mut B, arg: A);
}

impl<R, A> RenderWithArg<A> for &R
where
    R: RenderRefWithArg<A>,
{
    fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, arg: A) {
        self.render_ref_with_arg(rb, arg)
    }
}
