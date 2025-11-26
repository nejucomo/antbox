use speedy2d::color::Color;
use speedy2d::error::BacktraceError;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowCreationError, WindowHandler, WindowHelper,
};
use speedy2d::{Graphics2D, Window};

pub type Result<T> = std::result::Result<T, BacktraceError<WindowCreationError>>;

#[derive(Debug, Default)]
pub struct AntBox {}

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
        let size = helper.get_size_pixels().into_f32();
        let denom = 2f32;

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle(
            (size.x / denom, size.y / denom),
            size.magnitude() / denom / 5f32,
            Color::BLUE,
        );
        helper.request_redraw();
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
