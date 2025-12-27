use crate::RenderQueue;

/// Types which can enqueue render elements
pub trait Renderable {
    /// Schedule `self` to be rendered into `rq`
    fn render_to(self, rq: &mut RenderQueue);
}
