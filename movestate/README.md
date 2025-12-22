# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 
