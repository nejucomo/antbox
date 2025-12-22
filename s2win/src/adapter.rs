use derive_more::Unwrap;
use movestate::Slot;

use speedy2d::Graphics2D;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler,
    WindowHelper, WindowStartupInfo,
};

use crate::event::ButtonPosition::{self, Down, Up};
use crate::{WindowHandlerParams, WindowHandlerSimplified};

use self::Inner::{Pending, Started};

#[derive(Debug)]
pub(crate) struct HandlerAdapter<P, U>(Slot<Inner<P, U>>)
where
    P: WindowHandlerParams<U>;

#[derive(Debug, Unwrap)]
#[unwrap(ref, ref_mut)]
enum Inner<P, U>
where
    P: WindowHandlerParams<U>,
{
    Pending(P),
    Started(P::WHS),
}

impl<P, U> From<P> for HandlerAdapter<P, U>
where
    P: WindowHandlerParams<U>,
{
    fn from(param: P) -> Self {
        HandlerAdapter(Slot::from(Pending(param)))
    }
}

impl<P, U> WindowHandler<U> for HandlerAdapter<P, U>
where
    P: WindowHandlerParams<U>,
{
    fn on_start(&mut self, helper: &mut WindowHelper<U>, info: WindowStartupInfo) {
        self.0.map(|inner| {
            let p = inner.unwrap_pending();
            let s = p.start_handler(helper, info);
            Started(s)
        })
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<U>, user_event: U) {
        self.0
            .unwrap_started_mut()
            .on_user_event(helper, user_event)
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<U>, size_pixels: UVec2) {
        self.0.unwrap_started_mut().on_resize(helper, size_pixels)
    }

    fn on_mouse_grab_status_changed(&mut self, helper: &mut WindowHelper<U>, mouse_grabbed: bool) {
        self.0
            .unwrap_started_mut()
            .on_mouse_grab_status_changed(helper, mouse_grabbed)
    }

    fn on_fullscreen_status_changed(&mut self, helper: &mut WindowHelper<U>, fullscreen: bool) {
        self.0
            .unwrap_started_mut()
            .on_fullscreen_status_changed(helper, fullscreen)
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<U>, scale_factor: f64) {
        self.0
            .unwrap_started_mut()
            .on_scale_factor_changed(helper, scale_factor)
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<U>, graphics: &mut Graphics2D) {
        self.0.unwrap_started_mut().on_draw(helper, graphics)
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<U>, position: Vec2) {
        self.0.unwrap_started_mut().on_mouse_move(helper, position)
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<U>, button: MouseButton) {
        self.0
            .unwrap_started_mut()
            .on_mouse_button(helper, button, Down)
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<U>, button: MouseButton) {
        self.0
            .unwrap_started_mut()
            .on_mouse_button(helper, button, Up)
    }

    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<U>,
        distance: MouseScrollDistance,
    ) {
        self.0
            .unwrap_started_mut()
            .on_mouse_wheel_scroll(helper, distance)
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<U>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        on_key(
            self.0.unwrap_started_mut(),
            helper,
            virtual_key_code,
            scancode,
            Down,
        )
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<U>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        on_key(
            self.0.unwrap_started_mut(),
            helper,
            virtual_key_code,
            scancode,
            Up,
        )
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<U>, unicode_codepoint: char) {
        let _ = (helper, unicode_codepoint);
        todo!()
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<U>,
        state: ModifiersState,
    ) {
        let _ = (helper, state);
        todo!()
    }
}

fn on_key<S, P, U>(
    st: &mut S,
    helper: &mut WindowHelper<U>,
    virtual_key_code: Option<VirtualKeyCode>,
    scancode: KeyScancode,
    kpos: ButtonPosition,
) where
    S: WindowHandlerSimplified<P, U>,
{
    if let Some(vkc) = virtual_key_code {
        st.on_key(helper, vkc, kpos);
    } else {
        st.on_key_scancode(helper, scancode, kpos);
    }
}
