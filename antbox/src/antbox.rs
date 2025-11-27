use speedy2d::dimen::UVec2;
use speedy2d::window::{KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use crate::{ConwaysLife, Result, TickTimer, colors};

#[derive(Debug, Default)]
pub struct AntBox {
    prevwinsize: Option<UVec2>,
    tt: TickTimer,
    #[allow(dead_code)]
    conways: ConwaysLife,
}

impl AntBox {
    pub fn run() -> Result<()> {
        let w = Window::<()>::new_fullscreen_borderless(env!("CARGO_PKG_NAME"))?;
        w.run_loop(Self::default());
    }
}

impl AntBox {
    pub fn on_virtual_key_down(&mut self, helper: &mut WindowHelper<()>, vkc: VirtualKeyCode) {
        use VirtualKeyCode::Escape;

        match vkc {
            Escape => {
                log::info!("byte!");
                helper.terminate_loop();
            }
            _ => {
                // Ignore
            }
        }
    }
}

impl WindowHandler for AntBox {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        if self.tt.check_update() {
            let size = self.prevwinsize.get_or_insert_with(|| {
                let size = helper.get_size_pixels();
                log::debug!("window size: {size:?}");
                size
            });

            let fsize = size.into_f32();
            let denom = 2f32;

            graphics.clear_screen(colors::BACKGROUND);
            graphics.draw_circle(
                (fsize.x / denom, fsize.y / denom),
                fsize.magnitude() / denom / 5f32,
                colors::ANT,
            );
            helper.request_redraw();
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        ovkc: Option<VirtualKeyCode>,
        _: KeyScancode,
    ) {
        if let Some(keycode) = ovkc {
            self.on_virtual_key_down(helper, keycode)
        }
    }
}
