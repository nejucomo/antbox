use crate::Backend;

/// Types which can enqueue render elements
pub trait Renderable {
    /// Render `self` to be rendered into `rq`
    fn render_to<B: Backend>(self, rb: &mut B);
}

impl<X, Y> Renderable for (X, Y)
where
    X: Renderable,
    Y: Renderable,
{
    fn render_to<B: Backend>(self, rb: &mut B) {
        let (a, b) = self;
        a.render_to(rb);
        b.render_to(rb);
    }
}

impl<X, Y, Z> Renderable for (X, Y, Z)
where
    X: Renderable,
    Y: Renderable,
    Z: Renderable,
{
    fn render_to<B: Backend>(self, rb: &mut B) {
        let (a, b, c) = self;
        (a, (b, c)).render_to(rb);
    }
}
