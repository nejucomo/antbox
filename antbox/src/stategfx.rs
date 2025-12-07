use antbox_state::State;
use derive_more::{Deref, DerefMut, From};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vec2;

use crate::colors;

/// Encapsulate all graphics rendering for a [State]
#[derive(Debug, From, Deref, DerefMut)]
pub(crate) struct StateGfx(State);

impl StateGfx {
    pub(crate) fn draw(&self, graphics: &mut Graphics2D, viewsize: Vec2) {
        let bounds = self.0.food.bounds();
        let w32 = bounds.width as f32;
        let h32 = bounds.height as f32;
        let cellsize = Vec2::new(viewsize.x / w32, viewsize.y / h32);

        graphics.clear_screen(colors::BACKGROUND);

        self.draw_food_life(graphics, cellsize);
    }

    fn draw_food_life(&self, graphics: &mut Graphics2D, cellsize: Vec2) {
        for (pt, cell) in self.0.food.iter() {
            if cell.is_alive() {
                graphics.draw_circle(
                    (
                        cellsize.x * (pt.x() as f32) + cellsize.x / 2.0,
                        cellsize.y * (pt.y() as f32) + cellsize.y / 2.0,
                    ),
                    cellsize.x.min(cellsize.y) / 2.0,
                    colors::FOOD,
                );
            }
        }
    }
}
