use speedy2d::color::Color;

use crate::RenderCycle;

/// Types which can enqueue render elements
pub trait Renderable {
    /// Schedule `self` to be rendered into `rq`
    fn schedule(self, cycle: &mut RenderCycle);
}

impl Renderable for Color {
    fn schedule(self, cycle: &mut RenderCycle) {
        cycle.schedule_bg_color(self);
    }
}

impl<A, B> Renderable for (A, B)
where
    A: Renderable,
    B: Renderable,
{
    fn schedule(self, cycle: &mut RenderCycle) {
        let (a, b) = self;
        a.schedule(cycle);
        b.schedule(cycle);
    }
}

impl<A, B, C> Renderable for (A, B, C)
where
    A: Renderable,
    B: Renderable,
    C: Renderable,
{
    fn schedule(self, cycle: &mut RenderCycle) {
        let (a, b, c) = self;
        (a, (b, c)).schedule(cycle);
    }
}
