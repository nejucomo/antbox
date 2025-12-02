use antbox_state::State;
use derive_more::From;
use derive_new::new;

/// A [Notification] from the engine to the app
#[derive(Debug, From, new)]
pub enum Notification {
    /// A new antbox state is available
    NewState(State),
}
