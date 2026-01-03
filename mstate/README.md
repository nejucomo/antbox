# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Endless Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 

## Design Notes

- Blanket extensions use `B` as the blanket parameter.
- Blanket extensions always use textual copy of trait constraints, e.g. use `Self` not the blanket parameter `B`.
