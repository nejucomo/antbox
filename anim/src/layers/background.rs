use antbox_s2render::{RenderScheduler, Renderable};

use crate::colors;

/// The background layer
#[derive(Debug)]
pub struct Background;

impl Renderable for Background {
    fn schedule(self, rq: &mut RenderScheduler) {
        colors::DIRT.schedule(rq);
    }
}
