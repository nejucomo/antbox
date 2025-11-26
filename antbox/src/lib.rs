use speedy2d::color::Color;
use speedy2d::error::BacktraceError;
use speedy2d::window::{WindowCreationError, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

pub type Result<T> = std::result::Result<T, BacktraceError<WindowCreationError>>;

#[derive(Debug, Default)]
pub struct AntBox {}

impl AntBox {
    pub fn run() -> Result<()> {
        let window = Window::<()>::new_centered(env!("CARGO_PKG_NAME"), (640, 480))?;
        window.run_loop(Self::default());
        }
}

impl WindowHandler for AntBox {
    //     fn on_start(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         info: WindowStartupInfo,
    //     ) {
    //     }

    //     fn on_user_event(&mut self, helper: &mut WindowHelper<()>, user_event: ()) {}

    //     fn on_resize(
    //         &mut self,
    //         helper: &mut WindowHelper<()>, size_pixels: UVec2,
    //     ) {
    //     }

    //     fn on_mouse_grab_status_changed(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         mouse_grabbed: bool,
    //     ) {
    //     }

    //     fn on_fullscreen_status_changed(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         fullscreen: bool,
    //     ) {
    //     }

    //     fn on_scale_factor_changed(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         scale_factor: f64,
    //     ) {
    //     }

    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
        graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
        helper.request_redraw();
    }

    //     fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2)
    //     {

    //      }

    //     fn on_mouse_button_down(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         button: MouseButton
    //     )
    //     { }

    //     fn on_mouse_button_up(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         button: MouseButton
    //     )

    //
    //     fn on_mouse_wheel_scroll(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         distance: MouseScrollDistance
    //     )

    //     { }

    //     fn on_key_down(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         virtual_key_code: Option<VirtualKeyCode>,
    //         scancode: KeyScancode
    //     )

    //     { }

    //     fn on_key_up(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         virtual_key_code: Option<VirtualKeyCode>,
    //         scancode: KeyScancode
    //     )

    //     { }

    //     fn on_keyboard_char(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         unicode_codepoint: char

    //     )

    //
    //     fn on_keyboard_modifiers_changed(
    //         &mut self,
    //         helper: &mut WindowHelper<()>,
    //         state: ModifiersState
    //     )

    //     { }
}
