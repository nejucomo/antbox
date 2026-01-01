/// A [WindowEventHandler](crate::WindowEventHandler) responds to [WinEvent](crate::event::WinEvent)s with a [Control] to conrol windowing system
#[derive(Copy, Clone, Debug)]
pub enum Control {
    /// Cause the window system idle
    Idle,
    /// Request a redraw event
    RequestRedraw,
}
