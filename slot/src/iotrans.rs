/// A functional (move-semantics) transformation of a state based on an `Input` which produces an [Output](IOTransform::Output)
pub trait IOTransform<Input>: Sized {
    /// The output of a transform
    type Output;

    /// Transform the `self` state with `i` into a new [Self] state and an [Output](Self::Output)
    fn transform_io(self, i: Input) -> (Self, Self::Output);
}
