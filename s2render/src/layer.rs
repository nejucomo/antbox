/// The render layer
///
/// # [Ord] impl
///
/// [Layer] sorts with the _lower_ layers as _max_ to ensure they are drawn first in the [RenderQueue](crate::RenderQueue)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Layer {
    /// The highest layer, drawn last
    Hi = 0,
    /// The middle layer
    Mid = 1,
    /// The lowest layer, drawn first
    Low = 2,
}
