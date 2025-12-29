/// An error involving an [f32] value
#[derive(Copy, Clone, Debug)]
pub struct BoundedFloatError {
    /// The value in question
    pub f: f32,
}
