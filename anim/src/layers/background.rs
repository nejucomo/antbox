use antbox_render::{Backend, Renderable};

use crate::colors;

/// The background layer
#[derive(Debug)]
pub struct Background;

impl Renderable for Background {
    fn render_to<B: Backend>(self, rb: &mut B) {
        rb.clear_screen(colors::DIRT);
    }
}
