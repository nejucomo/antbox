//! Event-related API

use std::fmt::Debug;

use derive_debug::Dbg;
use derive_more::{From, IsVariant};
use speedy2d::Graphics2D;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{
    KeyScancode, ModifiersState, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowHelper,
};

/// An event with an associated [WindowHelper]
#[derive(Dbg)]
#[allow(missing_docs)]
pub struct WinEvent<'a, U: 'static> {
    #[dbg(placeholder = "...")]
    pub helper: &'a mut WindowHelper<U>,
    pub info: Info<'a, U>,
}

#[derive(Dbg, From)]
#[allow(missing_docs)]
pub enum Info<'a, U> {
    #[from(skip)]
    User(#[dbg(placeholder = "...")] U),
    OnDraw(#[dbg(placeholder = "...")] &'a mut Graphics2D),
    Input(Input),
}

#[derive(Debug)]
#[allow(missing_docs)]
pub enum Input {
    Resize(UVec2),
    FullscreenStatusChanged(bool),
    ScaleFactorChanged(f64),
    Mouse(MouseEvent),
    Key(KeyEvent),
    Unicode(char),
}

/// A mouse event
#[derive(Debug)]
#[allow(missing_docs)]
pub enum MouseEvent {
    Grabbed(bool),
    Move(Vec2),
    Button(MouseButton, ButtonPosition),
    WheelScroll(MouseScrollDistance),
}

#[derive(Debug, From)]
#[allow(missing_docs)]
pub enum KeyEvent {
    Virtual(ButtonPosition, VirtualKeyCode),
    Scancode(ButtonPosition, KeyScancode),
    ModifiersChanged(ModifiersState),
}

impl From<(Option<VirtualKeyCode>, KeyScancode, ButtonPosition)> for KeyEvent {
    fn from((ovkc, ksc, pos): (Option<VirtualKeyCode>, KeyScancode, ButtonPosition)) -> Self {
        use KeyEvent::{Scancode, Virtual};

        ovkc.map(|vkc| Virtual(pos, vkc))
            .unwrap_or(Scancode(pos, ksc))
    }
}

/// A button or key position
#[derive(Copy, Clone, Debug, IsVariant)]
#[allow(missing_docs)]
pub enum ButtonPosition {
    Up,
    Down,
}
