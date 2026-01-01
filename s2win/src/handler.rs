use antbox_geom::Dimensions;
use antbox_render::RenderRefWithArg;
use movestate::mutable::Update;
use speedy2d::window::WindowStartupInfo;

use crate::event::WinEvent;
use crate::{Control, UserEventSender};

/// A window handler API which makes a few simplifications over [speedy2d::window::WindowHandler]
pub trait WindowEventHandler<U>:
    Update<WinEvent<U>, Control> + RenderRefWithArg<Dimensions>
{
    /// The type of parameters used to start this handler
    type Params;

    /// Start this handler with both app [Self::Params] and [speedy2d::window] params
    fn start(params: Self::Params, ues: UserEventSender<U>, info: WindowStartupInfo) -> Self;
}
