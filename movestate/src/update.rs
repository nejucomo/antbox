use crate::Transform;

/// Any [Transform] with [Transform::Next] as `Self` is an [Update] for the same input `I`
pub trait Update<I>: Transform<I, Next = Self> {
    /// A synonym for [Self::transform]
    fn update(self, input: I) -> Self {
        self.transform(input)
    }
}

impl<B, I> Update<I> for B where B: Transform<I, Next = Self> {}
