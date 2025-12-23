# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Endless Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 

## Pulling Functions Apart and Putting Them Back Together

This crate defines a fine-grained family of traits and types by breaking apart the general function `P -> N` which takes a predecessor `P` and produces a "next" value `N`. We then aim to capture the "pieces" into an ergonomic rust API that allows useful abstractions depending on the kinds of pieces we combine.

### Inputness Axis

First, we decompose the predecessor `P` into two cases: `S` and `(S, I)` for whether or not the function domain is just a state/self `S` or also includes an input `I`:

- [TakeIntoNext] codifies `(S, I) -> N`.
- [IntoNext] codifies `S -> N`.

The [TakeIntoNext] trait is the "root" trait, from which the others in the family are derived via blanked impls. For example, [IntoNext] is a blanket extension for any [TakeIntoNext] over input type `()`. Put another way: implementors always implement [TakeIntoNext] and consumers choose which of the extension interfaces captures their constraints and provides the desired API.

### Next Axis

The possibilities for the [TakeIntoNext::Next] type provide the richness of the family, and all of the extension traits (so far) are based on [Next](TakeIntoNext::Next) types parameterized over `Self`:

- [TakeIntoUpdate] produces `Self` itself, codifying `(S, I) -> S` aka _Moore Machines_.
- [TakeIntoStarg] codifies `(S, I) -> (S, O)` aka _Mealy Machines_, using the [Starg] container.

For each `TakeInto-` trait in the Next Axis, there is an associated `Into-` extension trait which takes no input, and each of these extension traits is also an [IntoNext] extension:

- [IntoUpdate] = [TakeIntoUpdate] + [IntoNext]: `S -> S` aka an _endomorphism_
- [IntoStarg] = [TakeIntoStarg] + [IntoNext]: `S -> (S, O)` aka and _endless sequence_

## Design Notes

- All trait impls appear textually after the trait definition, starting with more general blankets to more specific, rather than next to the implementing type.
- Attempt the least-constrained bounds which check. For example, [TakeIntoUpdate] is not [Sized].
