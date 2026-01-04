/// An [S2App](crate::S2App) responds to app events or [WinEvent](crate::event::WinEvent)s with a [Control] to control windowing system
#[derive(Copy, Clone, Debug)]
pub enum Control {
    /// Cause the window system idle
    Idle,
    /// Request a redraw event
    RequestRedraw,
}
