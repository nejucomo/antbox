use speedy2d::color::Color;

use crate::RenderScheduler;

/// Types which can enqueue render elements
pub trait Renderable {
    /// Schedule `self` to be rendered into `rq`
    fn schedule(self, sched: &mut RenderScheduler);
}

impl Renderable for Color {
    fn schedule(self, sched: &mut RenderScheduler) {
        sched.schedule_bg_color(self);
    }
}

impl<A, B> Renderable for (A, B)
where
    A: Renderable,
    B: Renderable,
{
    fn schedule(self, sched: &mut RenderScheduler) {
        let (a, b) = self;
        a.schedule(sched);
        b.schedule(sched);
    }
}

impl<A, B, C> Renderable for (A, B, C)
where
    A: Renderable,
    B: Renderable,
    C: Renderable,
{
    fn schedule(self, sched: &mut RenderScheduler) {
        let (a, b, c) = self;
        (a, (b, c)).schedule(sched);
    }
}
