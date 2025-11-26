use pix_engine::color::Color;
use pix_engine::engine::PixEngine;
use pix_engine::error::Result;
use pix_engine::prelude::Font;
use pix_engine::state::PixState;

#[derive(Debug, Default)]
pub struct GameState {}

impl PixEngine for GameState {
    fn on_start(&mut self, ps: &mut PixState) -> Result<()> {
        ps.background(Color::GRAY);
        ps.font_family(Font::NOTO)?;
        ps.font_size(16)?;
        Ok(())
    }

    fn on_update(&mut self, ps: &mut PixState) -> Result<()> {
        if ps.mouse_pressed() {
            ps.fill(Color::ALICE_BLUE);
        } else {
            ps.fill(Color::DARK_KHAKI);
        }

        // Draw a circle with fill color at the mouse position with a radius of
        // 80.
        let m = ps.mouse_pos();
        ps.circle([m.x(), m.y(), 80])?;

        Ok(())
    }
}
