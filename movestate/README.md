# `movestate`

Building blocks for state evolution using move semantics; ex: Moore machines `(S, I) -> S`, Sequences `S -> (S, O)`, etc...

The goal is to enable judicious consumers to express precise types to prevent unnecessary state transitions at compile time, while more general consumers can operate over a broader range of providers. 

## [TakeIntoNext](takeinto::TakeIntoNext) Family Naming and Layout

Each trait in the [TakeIntoNext](takeinto::TakeIntoNext) family follow a consistent naming convention:

- If it takes an input it begins with a `TakeInto-` prefix and lives in [takeinto], otherwise it begins with a `Into-` prefix and lives in [into].
- If it may produce a `Self` value (anywhere within the [Next](takeinto::TakeIntoNext::Next) type, it has an `-Update-` infix.
- If it may also produce an output along with a `Self`, it has an `-Out` suffix.
- If it does not produce a `Self` specifically, it has a `-Next` suffix.

### The [into] Sub-family

| Trait | Shorthand | Comment |
|---|---|---|
| [IntoNext](into::IntoNext)                         | `S -> N`             | |

### The [takeinto] Sub-family

| Trait | Shorthand | Comment |
|---|---|---|
| [TakeIntoNext](takeinto::TakeIntoNext)                 | `(S, I) -> N`        | Providers for any [into]/[takeinto] consumer implement this most general "base" trait |
| [IntoOptUpdateWith](into::IntoOptUpdateWith)       | `(S, I) -> <(S, O)>` | |
| [IntoUpdateWith](into::IntoUpdateWith)             | `(S, I) -> S`        | |
| [IntoUpdateWithOutput](into::IntoUpdateWithOutput) | `(S, I) -> (S, O)`   | |
