use movestate::into::IntoUpdate;
use speedy2d::window::{WindowHelper, WindowStartupInfo};

use crate::event::WinEvent;

/// A window handler API which makes a few simplifications over [speedy2d::window::WindowHandler]
pub trait WindowEventHandler<U: 'static>: for<'a> IntoUpdate<WinEvent<'a, U>> {
    /// The type of parameters used to start this handler
    type Params;

    /// Start this handler with both app [Self::Params] and [speedy2d::window] params
    fn start(params: Self::Params, helper: &mut WindowHelper<U>, info: WindowStartupInfo) -> Self;
}
