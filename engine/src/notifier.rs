use antbox_cellauto::Generation;

use crate::Notification;
use crate::notification::NewFoodGeneration;

/// Applications provide a [Notifier] to the [Engine](crate::Engine) to receive engine updates
pub trait Notifier: Send {
    /// The error type `self` can return on failure to [Self::post]
    type Error: std::fmt::Display;

    /// Post a notification to the application
    fn post<T>(&self, notif: T) -> Result<(), Self::Error>
    where
        T: Into<Notification>;

    #[doc(hidden)]
    fn post_new_food_generation(&self, gencnt: usize, fg: Generation) -> Result<(), Self::Error> {
        self.post(NewFoodGeneration::new(gencnt, fg))
    }
}
