use movestate::Slot;

use speedy2d::Graphics2D;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler,
    WindowHelper, WindowStartupInfo,
};

use crate::WindowEventHandler;
use crate::event::ButtonPosition::{Down, Up};
use crate::event::Input::{
    FullscreenStatusChanged, Key, Mouse, Resize, ScaleFactorChanged, Unicode,
};
use crate::event::MouseInput::{Button, Grabbed, Move, WheelScroll};
use crate::event::{Info, WinEvent};
use crate::inner::AdapterInner;

pub(crate) struct HandlerAdapter<H, U>(Slot<AdapterInner<H, U>>)
where
    U: 'static,
    H: WindowEventHandler<U>;

impl<H, U> HandlerAdapter<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    pub fn new(params: H::Params) -> Self {
        HandlerAdapter(Slot::from(AdapterInner::new(params)))
    }

    fn update_event<'a, N>(&mut self, helper: &'a mut WindowHelper<U>, info: N)
    where
        N: Into<Info<'a, U>>,
    {
        self.0.update(WinEvent {
            helper,
            info: info.into(),
        });
    }
}

impl<H, U> WindowHandler<U> for HandlerAdapter<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    fn on_start(&mut self, helper: &mut WindowHelper<U>, info: WindowStartupInfo) {
        self.0.update((helper, info));
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<U>, uev: U) {
        self.update_event(helper, Info::User(uev));
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<U>, size_pixels: UVec2) {
        self.update_event(helper, Resize(size_pixels));
    }

    fn on_fullscreen_status_changed(&mut self, helper: &mut WindowHelper<U>, fullscreen: bool) {
        self.update_event(helper, FullscreenStatusChanged(fullscreen));
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<U>, factor: f64) {
        self.update_event(helper, ScaleFactorChanged(factor));
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<U>, graphics: &mut Graphics2D) {
        self.update_event(helper, graphics);
    }

    fn on_mouse_grab_status_changed(&mut self, helper: &mut WindowHelper<U>, grabbed: bool) {
        self.update_event(helper, Mouse(Grabbed(grabbed)));
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<U>, position: Vec2) {
        self.update_event(helper, Mouse(Move(position)));
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<U>, button: MouseButton) {
        self.update_event(helper, Mouse(Button(button, Down)));
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<U>, button: MouseButton) {
        self.update_event(helper, Mouse(Button(button, Up)));
    }

    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<U>,
        distance: MouseScrollDistance,
    ) {
        self.update_event(helper, Mouse(WheelScroll(distance)));
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<U>,
        ovkc: Option<VirtualKeyCode>,
        ksc: KeyScancode,
    ) {
        self.update_event(helper, Key((ovkc, ksc, Down).into()));
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<U>,
        ovkc: Option<VirtualKeyCode>,
        ksc: KeyScancode,
    ) {
        self.update_event(helper, Key((ovkc, ksc, Up).into()));
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<U>,
        state: ModifiersState,
    ) {
        self.update_event(helper, Key(state.into()));
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<U>, c: char) {
        self.update_event(helper, Unicode(c));
    }
}
