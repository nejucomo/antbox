use derive_more::IsVariant;
use derive_new::new;

/// An update to [AntboxAnimation](crate::AntboxAnimation)
#[derive(Debug, new)]
pub struct UpdateEvent<'r, R: rand::Rng> {
    /// The application rng
    pub rng: &'r mut R,
    /// The source of the update
    pub source: UpdateSource,
}

/// The source of an update
#[derive(Debug, IsVariant)]
pub enum UpdateSource {
    /// A clock tick update
    ClockTick,
    /// An explicit state update
    Step,
}
