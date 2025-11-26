use pix_engine::color::Color;
use pix_engine::engine::{Engine, PixEngine};
use pix_engine::error::Result;
use pix_engine::event::Key;
use pix_engine::prelude::Font;
use pix_engine::state::PixState;

#[derive(Debug, Default)]
pub struct AntBox {}

impl AntBox {
    pub fn run() -> Result<()> {
        let mut engine = Engine::builder()
            .title(env!("CARGO_PKG_NAME"))
            .target_frame_rate(5)
            .dimensions(800, 600)
            .fullscreen()
            .build()?;

        let mut app = AntBox::default();
        engine.run(&mut app)
    }
}

impl PixEngine for AntBox {
    fn on_start(&mut self, ps: &mut PixState) -> Result<()> {
        ps.background(Color::GRAY);
        ps.font_family(Font::NOTO)?;
        ps.font_size(16)?;
        Ok(())
    }

    fn on_update(&mut self, ps: &mut PixState) -> Result<()> {
        if ps.key_down(Key::Escape) {
            ps.quit();
        } else {
            // Draw a circle with fill color at the mouse position with a radius of
            // 80.
            let m = ps.mouse_pos();
            ps.circle([m.x(), m.y(), 80])?;
        }
        Ok(())
    }
}
