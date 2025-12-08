use antbox_engine::{Notification, Notifier};
use derive_more::From;
use speedy2d::window::UserEventSender;

#[derive(From)]
pub(crate) struct SpeedyNotifier(UserEventSender<Notification>);

impl Notifier for SpeedyNotifier {
    type Error = &'static str;

    fn post<T>(&self, notif: T) -> Result<(), Self::Error>
    where
        T: Into<Notification>,
    {
        self.0.send_event(notif.into()).map_err(|e| match e {
            speedy2d::window::EventLoopSendError::EventLoopNoLongerExists => {
                "event loop no longer exists"
            }
        })
    }
}

impl std::fmt::Debug for SpeedyNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SpeedyNotifier").field(&"..").finish()
    }
}
