mod gstate;

use pix_engine::engine::Engine;
use pix_engine::error::Result;

pub use self::gstate::GameState;

pub fn run() -> Result<()> {
    let mut engine = Engine::builder()
        .dimensions(800, 600)
        .title(env!("CARGO_PKG_NAME"))
        .show_frame_rate()
        .resizable()
        .build()?;

    let mut app = GameState::default();
    engine.run(&mut app)
}
