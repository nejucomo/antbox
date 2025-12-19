/// Like [TryInto] while ignoring the [TryInto::Error]
///
/// # Blanket impl
///
/// There is a blanket impl for every `B: TryInto<T>` that simply ignores the error
pub trait OptInto<T>: Sized + Copy {
    /// Attempt to convert `self` into `T`
    fn opt_into(self) -> Option<T>;
}

impl<B, T> OptInto<T> for B
where
    B: Sized + Copy + TryInto<T>,
{
    fn opt_into(self) -> Option<T> {
        self.try_into().ok()
    }
}
