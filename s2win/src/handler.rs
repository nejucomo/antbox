use std::fmt::Debug;

use speedy2d::Graphics2D;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHelper,
    WindowStartupInfo,
};

use crate::event::ButtonPosition;

pub trait WindowHandlerParams<U>: Sized + Debug {
    type WHS: WindowHandlerSimplified<Self, U>;

    fn start_handler(self, helper: &mut WindowHelper<U>, info: WindowStartupInfo) -> Self::WHS;
}

pub trait WindowHandlerSimplified<S, U>: Debug {
    fn on_user_event(&mut self, helper: &mut WindowHelper<U>, user_event: U);

    fn on_resize(&mut self, helper: &mut WindowHelper<U>, size_pixels: UVec2) {
        let _ = (helper, size_pixels);
    }

    fn on_mouse_grab_status_changed(&mut self, helper: &mut WindowHelper<U>, mouse_grabbed: bool) {
        let _ = (helper, mouse_grabbed);
    }

    fn on_fullscreen_status_changed(&mut self, helper: &mut WindowHelper<U>, fullscreen: bool) {
        let _ = (helper, fullscreen);
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<U>, scale_factor: f64) {
        let _ = (helper, scale_factor);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<U>, graphics: &mut Graphics2D);

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<U>, position: Vec2) {
        let _ = (helper, position);
    }

    fn on_mouse_button(
        &mut self,
        helper: &mut WindowHelper<U>,
        button: MouseButton,
        bpos: ButtonPosition,
    ) {
        let _ = (helper, button, bpos);
    }

    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<U>,
        distance: MouseScrollDistance,
    ) {
        let _ = (helper, distance);
    }

    fn on_key(&mut self, helper: &mut WindowHelper<U>, vkc: VirtualKeyCode, kpos: ButtonPosition) {
        let _ = (helper, vkc, kpos);
    }

    fn on_key_scancode(
        &mut self,
        helper: &mut WindowHelper<U>,
        scancode: KeyScancode,
        kpos: ButtonPosition,
    ) {
        let _ = (helper, scancode, kpos);
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<U>, unicode_codepoint: char) {
        let _ = (helper, unicode_codepoint);
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<U>,
        state: ModifiersState,
    ) {
        let _ = (helper, state);
    }
}
