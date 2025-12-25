# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Endless Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 

## Implementors

Providers impl [TakeIntoNext], while blanket extension `impl`s allow consumers to select any trait in the family. For example:

```
mod provider {
    // The producer code only impls `TakeIntoNext`:
    use movestate::TakeIntoNext;

    /// A sequence of naturals, e.g. `1..`
    #[derive(Copy, Clone, Default)]
    pub struct Naturals(usize);

    impl TakeIntoNext<()> for Naturals {
        type Next = (Self, usize);

        fn take_into_next(self, (): ()) -> (Self, usize) {
            let next = Naturals(self.0 + 1);
            (next, next.0)
        }
    }
}

fn consumer() {
    // The consumer code only uses `IntoStout`:
    use movestate::stout::IntoStout;

    let s0 = provider::Naturals::default();
    let (s1, n1) = s0.into_self_out();
    let (s2, n2) = s1.into_self_out();
    let (_, n3) = s2.into_self_out();

    assert_eq!(n1, 1);
    assert_eq!(n2, 2);
    assert_eq!(n3, 3);
}
```


## Design Notes

- All trait impls appear textually after the trait definition, starting with more general blankets to more specific, rather than next to the implementing type.
- Attempt the least-constrained bounds which check. For example, [TakeIntoNext] is not [Sized].
