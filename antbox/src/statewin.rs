use antbox_state::State;
use derive_more::{Deref, DerefMut, From};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::colors;

#[derive(Debug, From, Deref, DerefMut)]
pub(crate) struct StateWin(State);

impl StateWin {
    pub(crate) fn draw(&self, graphics: &mut Graphics2D, viewsize: Vec2) {
        let bounds = self.0.food.bounds();
        let w32 = bounds.width as f32;
        let h32 = bounds.height as f32;
        let cell_width = viewsize.x / w32;
        let cell_height = viewsize.y / h32;

        graphics.clear_screen(colors::BACKGROUND);

        log::debug!("drawing popped food grid");
        for (pt, cell) in self.0.food.iter() {
            if cell.is_alive() {
                graphics.draw_circle(
                    (
                        cell_width * (pt.x() as f32) + cell_width / 2.0,
                        cell_height * (pt.y() as f32) + cell_height / 2.0,
                    ),
                    cell_width.min(cell_height) / 2.0,
                    colors::FOOD,
                );
            }
        }
    }
}
