use thiserror::Error;

/// An error involving an [f32] value
#[derive(Copy, Clone, Debug, Error)]
#[error("BoundedFloatError for {f:?}")]
pub struct BoundedFloatError {
    /// The value in question
    pub f: f32,
}
