use crate::Transform;

/// Any [Transform] with [Transform::Next] as `Option<Self>` is an [OptUpdate] for the same input `I`
pub trait OptUpdate<I>: Transform<I, Next = Option<Self>> {
    /// A synonym for [Self::transform]
    fn opt_update(self, input: I) -> Option<Self> {
        self.transform(input)
    }
}

impl<B, I> OptUpdate<I> for B where B: Transform<I, Next = Option<Self>> {}
