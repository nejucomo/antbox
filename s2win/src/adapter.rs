use antbox_render::RenderWithArg as _;
use antbox_s2render::Speedy2Backend;
use moveslot::{MapInPlace as _, MoveSlot};
use mstate::Responder;
use speedy2d::Graphics2D;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHandler,
    WindowHelper, WindowStartupInfo,
};

use crate::event::ButtonPosition::{Down, Up};
use crate::event::MouseInput::{Button, Grabbed, Move, WheelScroll};
use crate::event::WinEvent::{
    FullscreenStatusChanged, Key, Mouse, Resize, ScaleFactorChanged, Unicode,
};
use crate::inner::AdapterInner;
use crate::into_ab::IntoAntbox as _;
use crate::{Control, S2App};

pub(crate) struct HandlerAdapter<A>(MoveSlot<AdapterInner<A>>)
where
    A: S2App;

impl<A> HandlerAdapter<A>
where
    A: S2App,
{
    pub fn new(params: A::Params) -> Self {
        HandlerAdapter(MoveSlot::from(AdapterInner::new(params)))
    }

    fn dispatch_event<V>(&mut self, helper: &mut WindowHelper<A::Event>, ev: V)
    where
        A: Responder<V, Response = Control>,
    {
        use Control::*;

        match self.0.handle(ev) {
            Idle => {}
            RequestRedraw => helper.request_redraw(),
        }
    }
}

impl<A> WindowHandler<A::Event> for HandlerAdapter<A>
where
    A: S2App,
{
    fn on_start(&mut self, helper: &mut WindowHelper<A::Event>, info: WindowStartupInfo) {
        let ues = helper.create_user_event_sender();
        self.0
            .map_in_place(|inner| inner.start_transition(ues, info));
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<A::Event>, graphics: &mut Graphics2D) {
        let view_size = helper.get_size_pixels().into_antbox();
        let mut s2b = Speedy2Backend::from(graphics);
        let inner: &AdapterInner<_> = &self.0;
        inner.render_with_arg(&mut s2b, view_size);
    }

    // The rest of events are dispatched to the app:
    fn on_user_event(&mut self, helper: &mut WindowHelper<A::Event>, uev: A::Event) {
        self.dispatch_event(helper, uev);
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<A::Event>, size_pixels: UVec2) {
        self.dispatch_event(helper, Resize(size_pixels.into_antbox()));
    }

    fn on_fullscreen_status_changed(
        &mut self,
        helper: &mut WindowHelper<A::Event>,
        fullscreen: bool,
    ) {
        self.dispatch_event(helper, FullscreenStatusChanged(fullscreen));
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<A::Event>, factor: f64) {
        self.dispatch_event(helper, ScaleFactorChanged(factor));
    }

    fn on_mouse_grab_status_changed(&mut self, helper: &mut WindowHelper<A::Event>, grabbed: bool) {
        self.dispatch_event(helper, Mouse(Grabbed(grabbed)));
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<A::Event>, position: Vec2) {
        self.dispatch_event(helper, Mouse(Move(position.into_antbox())));
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<A::Event>, button: MouseButton) {
        self.dispatch_event(helper, Mouse(Button(button, Down)));
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<A::Event>, button: MouseButton) {
        self.dispatch_event(helper, Mouse(Button(button, Up)));
    }

    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<A::Event>,
        distance: MouseScrollDistance,
    ) {
        self.dispatch_event(helper, Mouse(WheelScroll(distance)));
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<A::Event>,
        ovkc: Option<VirtualKeyCode>,
        ksc: KeyScancode,
    ) {
        self.dispatch_event(helper, Key((ovkc, ksc, Down).into()));
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<A::Event>,
        ovkc: Option<VirtualKeyCode>,
        ksc: KeyScancode,
    ) {
        self.dispatch_event(helper, Key((ovkc, ksc, Up).into()));
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<A::Event>,
        state: ModifiersState,
    ) {
        self.dispatch_event(helper, Key(state.into()));
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<A::Event>, c: char) {
        self.dispatch_event(helper, Unicode(c));
    }
}
