//! Events to be handled

use std::fmt::Debug;

use antbox_geom::{Dimensions, Point};
use antbox_s2render::Speedy2Backend;
use derive_debug::Dbg;
use derive_more::{From, IsVariant};
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

/// Top-level event info, aside from the [WindowHelper]
#[derive(Dbg, From)]
pub enum Info<'a, U> {
    /// An application-specified user event
    #[from(skip)]
    User(#[dbg(placeholder = "...")] U),
    /// A request to draw the window from the framework
    DrawRequest(#[dbg(placeholder = "...")] Speedy2Backend<'a>),
    /// Input event information
    Input(Input),
}

/// A pure input event
#[derive(Debug)]
pub enum Input {
    /// The window was resized
    Resize(Dimensions),
    /// The window's fullscreen status changed
    FullscreenStatusChanged(bool),
    /// The window's scale factor changed
    ScaleFactorChanged(f64),
    /// A mouse input event
    Mouse(MouseInput),
    /// A key input event
    Key(KeyInput),
    /// A unicode character input event
    Unicode(char),
}

/// A mouse input event
#[derive(Debug)]
pub enum MouseInput {
    /// The mouse cursor was (un-)grabbed
    Grabbed(bool),
    /// The mouse moved
    Move(Point),
    /// A mouse button changed position
    Button(MouseButton, ButtonPosition),
    /// A mouse scroll wheel changed position
    WheelScroll(MouseScrollDistance),
}

/// A key input event
#[derive(Debug, From)]
pub enum KeyInput {
    /// A key represented by a [VirtualKeyCode] changed position
    Virtual(ButtonPosition, VirtualKeyCode),
    /// A key represented by a [KeyScancode] (and unrepresentable by a [VirtualKeyCode]) changed position
    Scancode(ButtonPosition, KeyScancode),
    /// Key modifiers changed state
    ModifiersChanged(ModifiersState),
}

impl From<(Option<VirtualKeyCode>, KeyScancode, ButtonPosition)> for KeyInput {
    fn from((ovkc, ksc, pos): (Option<VirtualKeyCode>, KeyScancode, ButtonPosition)) -> Self {
        use KeyInput::{Scancode, Virtual};

        ovkc.map(|vkc| Virtual(pos, vkc))
            .unwrap_or(Scancode(pos, ksc))
    }
}

/// A button or key position
#[derive(Copy, Clone, Debug, IsVariant)]
pub enum ButtonPosition {
    /// The button/key changed position to up
    Up,
    /// The button/key changed position to down
    Down,
}
