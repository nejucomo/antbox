# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Endless Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 

## The [movestate](crate) Trait Family

A summary of the [movestate](crate) Trait Family:

| Trait | Shorthand | Comment |
|---|---|---|
| [IntoNext]       | `S -> N`           | most general input-less transition                 |
| [IntoUpdate]     | `S -> S`           | a.k.a _state transition function_ / _endomorphism_ |
| [IntoStarg]      | `S -> (S, O)`      | a.k.a _endless sequence_                           |
| [TakeIntoNext]   | `(S, I) -> N`      | most general trait; impl base trait                |
| [TakeIntoUpdate] | `(S, I) -> S`      | a.k.a _Moore machine_                              |
| [TakeIntoStarg]  | `(S, I) -> (S, O)` | a.k.a _Mealy machine_                              |

### Implementing

Implementors always implement [TakeIntoNext], while blanket extension `impl`s allow consumers to select any trait in the family:

| Consumer | Implementor |
|---|---|
| [IntoNext]       | `TakeIntoNext<()>`                          |
| [IntoUpdate]     | `TakeIntoNext<(), Next: Into<Self>>`        |
| [IntoStarg]      | `TakeIntoNext<(), Next: Into<Starg<S, O>>>` |
| [TakeIntoNext]   | [TakeIntoNext]                              |
| [TakeIntoUpdate] | `TakeIntoNext<I, Next: Into<Self>>`         |
| [TakeIntoStarg]  | `TakeIntoNext<I, Next: Into<Starg<S, O>>>`  |

## Design Notes

- All trait impls appear textually after the trait definition, starting with more general blankets to more specific, rather than next to the implementing type.
- Attempt the least-constrained bounds which check. For example, [TakeIntoUpdate] is not [Sized].
