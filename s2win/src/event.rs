//! Event-related API
use derive_more::IsVariant;

/// A button or key position
#[derive(Debug, IsVariant)]
#[allow(missing_docs)]
pub enum ButtonPosition {
    Up,
    Down,
}
