use speedy2d::Graphics2D;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler,
    WindowHelper, WindowStartupInfo,
};

use crate::AntBox;

#[derive(Debug, Default)]
pub struct AntBoxWindow(Option<AntBox>);

#[allow(unused_variables)]
impl WindowHandler<()> for AntBoxWindow {
    fn on_start(&mut self, helper: &mut WindowHelper<()>, info: WindowStartupInfo) {
        self.0.as_mut().map(|ab| ab.on_start(helper, info));
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<()>, user_event: ()) {
        self.0
            .as_mut()
            .map(|ab| ab.on_user_event(helper, user_event));
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.0.as_mut().map(|ab| ab.on_resize(helper, size_pixels));
    }

    fn on_mouse_grab_status_changed(&mut self, helper: &mut WindowHelper<()>, mouse_grabbed: bool) {
        self.0
            .as_mut()
            .map(|ab| ab.on_mouse_grab_status_changed(helper, mouse_grabbed));
    }

    fn on_fullscreen_status_changed(&mut self, helper: &mut WindowHelper<()>, fullscreen: bool) {
        self.0
            .as_mut()
            .map(|ab| ab.on_fullscreen_status_changed(helper, fullscreen));
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<()>, scale_factor: f64) {
        self.0
            .as_mut()
            .map(|ab| ab.on_scale_factor_changed(helper, scale_factor));
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        self.0.as_mut().map(|ab| ab.on_draw(helper, graphics));
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {
        self.0.as_mut().map(|ab| ab.on_mouse_move(helper, position));
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        self.0
            .as_mut()
            .map(|ab| ab.on_mouse_button_down(helper, button));
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        self.0
            .as_mut()
            .map(|ab| ab.on_mouse_button_up(helper, button));
    }

    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<()>,
        distance: MouseScrollDistance,
    ) {
        self.0
            .as_mut()
            .map(|ab| ab.on_mouse_wheel_scroll(helper, distance));
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        self.0
            .as_mut()
            .map(|ab| ab.on_key_down(helper, virtual_key_code, scancode));
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        self.0
            .as_mut()
            .map(|ab| ab.on_key_up(helper, virtual_key_code, scancode));
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<()>, unicode_codepoint: char) {
        self.0
            .as_mut()
            .map(|ab| ab.on_keyboard_char(helper, unicode_codepoint));
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<()>,
        modifiers: ModifiersState,
    ) {
        self.0
            .as_mut()
            .map(|ab| ab.on_keyboard_modifiers_changed(helper, modifiers));
    }
}
