use crate::Notification;

/// Applications provide a [Notifier] to the engine to receive engine updates
pub trait Notifier: Send {
    /// The error type `self` can return on failure to [Self::post]
    type Error: std::fmt::Display;

    /// Post a notification to the application
    fn post<T>(&self, notif: T) -> Result<(), Self::Error>
    where
        T: Into<Notification>;
}
