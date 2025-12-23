# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Endless Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 

## The [movestate](crate) Trait Family

A summary of the [movestate](crate) Trait Family:

| Trait | Shorthand | Comment |
|---|---|---|
| [into::IntoNext]               | `S -> N`               | most general input-less transition                 |
| [into::IntoUpdate]             | `S -> S`               | a.k.a _state transition function_ / _endomorphism_ |
| [into::IntoStarg]              | `S -> (S, O)`          | a.k.a _endless sequence_                           |
| [into::IntoTermStarg]          | `S -> (S, O) / T`      | a.k.a _terminating sequence_                       |
| [take_into::TakeIntoNext]      | `(S, I) -> N`          | most general trait; impl base trait                |
| [take_into::TakeIntoUpdate]    | `(S, I) -> S`          | a.k.a _Moore machine_                              |
| [take_into::TakeIntoStarg]     | `(S, I) -> (S, O)`     | a.k.a _Mealy machine_                              |
| [take_into::TakeIntoTermStarg] | `(S, I) -> (S, O) / T` | a.k.a _terminating Mealy machine_                  |

### Implementing

Implementors always implement [take_into::TakeIntoNext], while blanket extension `impl`s allow consumers to select any trait in the family:

| Consumer | Implementor |
|---|---|
| [into::IntoNext]               | `TakeIntoNext<()>`                              |
| [into::IntoUpdate]             | `TakeIntoNext<(), Next: Into<Self>>`            |
| [into::IntoStarg]              | `TakeIntoNext<(), Next: Into<Starg<S, O>>>`     |
| [into::IntoTermStarg]          | `TakeIntoNext<(), Next: Into<TermStarg<S, O>>>` |
| [take_into::TakeIntoNext]      | [take_into::TakeIntoNext]                       |
| [take_into::TakeIntoUpdate]    | `TakeIntoNext<I, Next: Into<Self>>`             |
| [take_into::TakeIntoStarg]     | `TakeIntoNext<I, Next: Into<Starg<S, O>>>`      |
| [take_into::TakeIntoTermStarg] | `TakeIntoNext<I, Next: Into<TermStarg<S, O>>>`  |

## Design Notes

- All trait impls appear textually after the trait definition, starting with more general blankets to more specific, rather than next to the implementing type.
- Attempt the least-constrained bounds which check. For example, [take_into::TakeIntoUpdate] is not [Sized].
