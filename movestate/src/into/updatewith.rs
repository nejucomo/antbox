use crate::into::IntoUpdateWithOutput;

/// Update `Self` with `I` into a new `Self`
///
/// This is also known as a _Moore Machine_.
pub trait IntoUpdateWith<I>: IntoUpdateWithOutput<I, ()> {
    /// Update `self` with `I` into a new `Self`
    fn into_update_with(self, input: I) -> Self {
        let (next, ()) = self.into_update_output(input);
        next
    }
}
