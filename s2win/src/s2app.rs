use antbox_geom::Dimensions;
use antbox_render::RenderRefWithArg;
use mstate::Responder;
use speedy2d::Window;
use speedy2d::error::BacktraceError;
use speedy2d::window::{WindowCreationError, WindowCreationOptions};

use crate::Control;
use crate::adapter::HandlerAdapter;
use crate::event::{InitEvent, WinEvent};

/// Handle events and draw to the window
pub trait S2App:
    'static
    + From<InitEvent<Self::Params, Self::Event>>
    + Responder<Self::Event, Response = Control>
    + Responder<WinEvent, Response = Control>
    + RenderRefWithArg<Dimensions>
{
    /// The app's name as shown in the windowing system
    const APP_NAME: &'static str;

    /// The app's initialization parameters
    type Params;

    /// The app-specific event type
    type Event: 'static;

    /// Launch the app with the given `params`; on success this never returns
    fn launch(params: Self::Params) -> Result<(), BacktraceError<WindowCreationError>> {
        let w = Window::new_with_user_events(
            env!("CARGO_PKG_NAME"),
            WindowCreationOptions::new_fullscreen_borderless(),
        )?;

        w.run_loop(HandlerAdapter::<Self>::new(params))
    }
}
