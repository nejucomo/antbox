# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 

## The [into] Family

| Trait | Shorthand | Comment |
|---|---|---|
| [IntoNextWith](into::IntoNextWith)                 | `(S, I) -> N`        | Providers implement this; most general |
| [IntoNext](into::IntoNext)                         | `S -> N`             | |
| [IntoOptUpdateWith](into::IntoOptUpdateWith)       | `(S, I) -> <(S, O)>` | |
| [IntoUpdateWith](into::IntoUpdateWith)             | `(S, I) -> S`        | |
| [IntoUpdateWithOutput](into::IntoUpdateWithOutput) | `(S, I) -> (S, O)`   | |

### Naming System for [into]

The traits in the [into] family follow a consistent naming convention:

- If it takes an input it begins with a `TakeInto-` prefix, otherwise it begins with a `Into-` prefix.
- If it may produce a `Self` value (anywhere within the [Next](into::IntoNextWith::Next) type, it has an `-Update-` infix.
- If it may also produce an output along with a `Self`, it has an `-Out` suffix.
- If it does not produce a `Self` specifically, it has a `-Next` suffix.
