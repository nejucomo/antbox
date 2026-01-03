use antbox_render::RenderWithArg as _;
use antbox_s2render::Speedy2Backend;
use moveslot::MoveSlot;
use mstate::mutable::Update as _;
use speedy2d::Graphics2D;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler,
    WindowHelper, WindowStartupInfo,
};

use crate::event::ButtonPosition::{Down, Up};
use crate::event::MouseInput::{Button, Grabbed, Move, WheelScroll};
use crate::event::WinEvent::{
    self, FullscreenStatusChanged, Key, Mouse, Resize, ScaleFactorChanged, Unicode,
};
use crate::inner::AdapterInner;
use crate::into_ab::IntoAntbox as _;
use crate::{Control, WindowEventHandler};

pub(crate) struct HandlerAdapter<H, U>(MoveSlot<AdapterInner<H, U>>)
where
    U: 'static,
    H: WindowEventHandler<U>;

impl<H, U> HandlerAdapter<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    pub fn new(params: H::Params) -> Self {
        HandlerAdapter(MoveSlot::from(AdapterInner::new(params)))
    }

    fn dispatch_event<N>(&mut self, helper: &mut WindowHelper<U>, info: N)
    where
        N: Into<WinEvent<U>>,
    {
        use Control::*;

        match self.0.update(info.into()) {
            Idle => {}
            RequestRedraw => helper.request_redraw(),
        }
    }
}

// impl<H, U> RenderWithArg<Dimensions> for HandlerAdapter<H, U>
// where
//     U: 'static,
//     H: WindowEventHandler<U>,
// {
//     fn render_with_arg<B: ?Sized + Backend>(self, rb: &mut B, dims: Dimensions) {
//         self.0.render_with_arghjkkkj
//     }
// }

impl<H, U> WindowHandler<U> for HandlerAdapter<H, U>
where
    U: 'static,
    H: WindowEventHandler<U>,
{
    fn on_start(&mut self, helper: &mut WindowHelper<U>, info: WindowStartupInfo) {
        let ues = helper.create_user_event_sender();
        self.0.update((ues, info));
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<U>, graphics: &mut Graphics2D) {
        let view_size = helper.get_size_pixels().into_antbox();
        let mut s2b = Speedy2Backend::from(graphics);
        let inner: &AdapterInner<_, _> = &self.0;
        inner.render_with_arg(&mut s2b, view_size);
    }

    // The rest of events are dispatched to the app:
    fn on_user_event(&mut self, helper: &mut WindowHelper<U>, uev: U) {
        self.dispatch_event(helper, WinEvent::User(uev));
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<U>, size_pixels: UVec2) {
        self.dispatch_event(helper, Resize(size_pixels.into_antbox()));
    }

    fn on_fullscreen_status_changed(&mut self, helper: &mut WindowHelper<U>, fullscreen: bool) {
        self.dispatch_event(helper, FullscreenStatusChanged(fullscreen));
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<U>, factor: f64) {
        self.dispatch_event(helper, ScaleFactorChanged(factor));
    }

    fn on_mouse_grab_status_changed(&mut self, helper: &mut WindowHelper<U>, grabbed: bool) {
        self.dispatch_event(helper, Mouse(Grabbed(grabbed)));
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<U>, position: Vec2) {
        self.dispatch_event(helper, Mouse(Move(position.into_antbox())));
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<U>, button: MouseButton) {
        self.dispatch_event(helper, Mouse(Button(button, Down)));
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<U>, button: MouseButton) {
        self.dispatch_event(helper, Mouse(Button(button, Up)));
    }

    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<U>,
        distance: MouseScrollDistance,
    ) {
        self.dispatch_event(helper, Mouse(WheelScroll(distance)));
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<U>,
        ovkc: Option<VirtualKeyCode>,
        ksc: KeyScancode,
    ) {
        self.dispatch_event(helper, Key((ovkc, ksc, Down).into()));
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<U>,
        ovkc: Option<VirtualKeyCode>,
        ksc: KeyScancode,
    ) {
        self.dispatch_event(helper, Key((ovkc, ksc, Up).into()));
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<U>,
        state: ModifiersState,
    ) {
        self.dispatch_event(helper, Key(state.into()));
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<U>, c: char) {
        self.dispatch_event(helper, Unicode(c));
    }
}
