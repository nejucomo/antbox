use std::fmt::Debug;

use speedy2d::Graphics2D;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHelper,
    WindowStartupInfo,
};

use crate::event::ButtonPosition;

/// A set of parameters used to initialize a [WindowHandlerSimplified]
pub trait WindowHandlerParams<U>: Sized + Debug {
    /// The associated handler type
    type WHS: WindowHandlerSimplified<U>;

    /// Construct the handler with `self` params along with `helper` and `info`
    fn start_handler(self, helper: &mut WindowHelper<U>, info: WindowStartupInfo) -> Self::WHS;
}

/// A window handler API which makes a few simplifications over [speedy2d::window::WindowHandler]
pub trait WindowHandlerSimplified<U>: Debug {
    /// Identical to [speedy2d::window::WindowHandler::on_user_event]
    fn on_user_event(&mut self, helper: &mut WindowHelper<U>, user_event: U);

    /// Identical to [speedy2d::window::WindowHandler::on_resize]
    fn on_resize(&mut self, helper: &mut WindowHelper<U>, size_pixels: UVec2) {
        let _ = (helper, size_pixels);
    }

    /// Identical to [speedy2d::window::WindowHandler::on_mouse_grab_status_changed]
    fn on_mouse_grab_status_changed(&mut self, helper: &mut WindowHelper<U>, mouse_grabbed: bool) {
        let _ = (helper, mouse_grabbed);
    }

    /// Identical to [speedy2d::window::WindowHandler::on_fullscreen_status_changed]
    fn on_fullscreen_status_changed(&mut self, helper: &mut WindowHelper<U>, fullscreen: bool) {
        let _ = (helper, fullscreen);
    }

    /// Identical to [speedy2d::window::WindowHandler::on_scale_factor_changed]
    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<U>, scale_factor: f64) {
        let _ = (helper, scale_factor);
    }

    /// Identical to [speedy2d::window::WindowHandler::on_draw]
    fn on_draw(&mut self, helper: &mut WindowHelper<U>, graphics: &mut Graphics2D);

    /// Identical to [speedy2d::window::WindowHandler::on_mouse_move]
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<U>, position: Vec2) {
        let _ = (helper, position);
    }

    /// Similar to [speedy2d::window::WindowHandler::on_mouse_button_up]/[…_down](speedy2d::window::WindowHandler::on_mouse_button_down) multiplexed with [ButtonPosition]
    fn on_mouse_button(
        &mut self,
        helper: &mut WindowHelper<U>,
        button: MouseButton,
        bpos: ButtonPosition,
    ) {
        let _ = (helper, button, bpos);
    }

    /// Identical to [speedy2d::window::WindowHandler::on_mouse_wheel_scroll]
    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<U>,
        distance: MouseScrollDistance,
    ) {
        let _ = (helper, distance);
    }

    /// Handle a key moving into a [ButtonPosition] for a [VirtualKeyCode]
    ///
    /// If this is called, [Self::on_key_scancode] will *not* be called.
    fn on_key(&mut self, helper: &mut WindowHelper<U>, vkc: VirtualKeyCode, kpos: ButtonPosition) {
        let _ = (helper, vkc, kpos);
    }

    /// Handle a key moving into a [ButtonPosition] for a [KeyScancode]
    ///
    /// This is *only* called if there's no associated [VirtualKeyCode]
    fn on_key_scancode(
        &mut self,
        helper: &mut WindowHelper<U>,
        scancode: KeyScancode,
        kpos: ButtonPosition,
    ) {
        let _ = (helper, scancode, kpos);
    }

    /// Identical to [speedy2d::window::WindowHandler::on_keyboard_char]
    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<U>, unicode_codepoint: char) {
        let _ = (helper, unicode_codepoint);
    }

    /// Identical to [speedy2d::window::WindowHandler::on_keyboard_modifiers_changed]
    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<U>,
        state: ModifiersState,
    ) {
        let _ = (helper, state);
    }
}
