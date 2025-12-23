# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Endless Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 

## The [movestate](crate) Trait Family

| Trait | Shorthand | [Next](TakeIntoNext::Next): `Into<_>` constraints | Comment |
|---|---|---|---|
| [IntoNext]   | `S -> N`      | _any_         | most general input-less transition |
| [IntoUpdate] | `S -> S`      | `Self`        | a.k.a _state transition function_ / _endomorphism_ |
| [IntoStarg]  | `S -> (S, O)` | `Starg<S, O>` | a.k.a _endless sequence_ |
| [TakeIntoNext]   | `(S, I) -> N`      | _any_         | most general trait; impl base trait |
| [TakeIntoUpdate] | `(S, I) -> S`      | `Self`        | a.k.a _Moore machine_ |
| [TakeIntoStarg]  | `(S, I) -> (S, O)` | `Starg<S, O>` | a.k.a _Mealy machine_ |

## Design Notes

- All trait impls appear textually after the trait definition, starting with more general blankets to more specific, rather than next to the implementing type.
- Attempt the least-constrained bounds which check. For example, [TakeIntoUpdate] is not [Sized].
