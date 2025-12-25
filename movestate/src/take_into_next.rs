/// `(S, I) -> N`
///
/// The base implementation trait which defines the [movestate](crate) family.
///
/// ## Implementors
///
/// Providers impl [TakeIntoNext], while blanket extension `impl`s allow consumers to select any trait in the family. For example:
///
/// ```
/// mod provider {
///     // The producer code only impls `TakeIntoNext`:
///     use movestate::TakeIntoNext;
///
///     /// A sequence of naturals, e.g. `1..`
///     #[derive(Copy, Clone, Default)]
///     pub struct Naturals(usize);
///
///     impl TakeIntoNext<()> for Naturals {
///         type Next = (Self, usize);
///
///         fn take_into_next(self, (): ()) -> (Self, usize) {
///             let next = Naturals(self.0 + 1);
///             (next, next.0)
///         }
///     }
/// }
///
/// fn consumer() {
///     // The consumer code only uses `IntoStout`:
///     use movestate::IntoStout;
///
///     let s0 = provider::Naturals::default();
///     let (s1, n1) = s0.into_self_out();
///     let (s2, n2) = s1.into_self_out();
///     let (_, n3) = s2.into_self_out();
///
///     assert_eq!(n1, 1);
///     assert_eq!(n2, 2);
///     assert_eq!(n3, 3);
/// }
/// ```
///
///
/// ## Design Notes
///
/// - All trait impls appear textually after the trait definition, starting with more general blankets to more specific, rather than next to the implementing type.
/// - Attempt the least-constrained bounds which check. For example, [TakeIntoNext] is not [Sized].
pub trait TakeIntoNext<I> {
    /// The next type produced when processing an `input`
    type Next;

    /// Take `self` and an `input` into a [Self::Next] value
    fn take_into_next(self, input: I) -> Self::Next;
}
