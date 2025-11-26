use pix_engine::engine::Engine;
use pix_engine::error::Result;

use crate::GameState;

pub fn run() -> Result<()> {
    let mut engine = Engine::builder()
        .title(env!("CARGO_PKG_NAME"))
        .target_frame_rate(5)
        .dimensions(800, 600)
        .build()?;

    let mut app = GameState::default();
    engine.run(&mut app)
}
