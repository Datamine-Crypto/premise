# premise_macros

[![crates.io](https://img.shields.io/crates/v/premise_macros.svg)](https://crates.io/crates/premise_macros)
[![docs.rs](https://img.shields.io/docsrs/premise_macros)](https://docs.rs/premise_macros)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Datamine-Crypto/premise/blob/main/LICENSE)

**The reason macros.** In Premise a value that somebody chose carries the reason they chose it, as
code rather than as a comment. These are the macros that attach one.

```rust
use premise_macros::{because, source};

pub struct WearTest;
source!(WearTest, "the hinge fatigue test run on the entry door before fit-out");

pub const OPEN_LIMIT: u32 = 3;
because!(OPEN_LIMIT, WearTest, "three openings is where the test showed hinge fatigue");
```

| macro | what it states |
|---|---|
| `because!(ITEM, [SOURCE,] "...")` | why this item is what it is, optionally citing a source |
| `source!(NAME, "...")` | a thing a reason may cite: a test, a measurement, a person's decision |
| `provisional!(ITEM, [SOURCE,] "...")` | nobody has decided yet, and this says what would settle it |
| `decided!(TYPE, TRAIT, "...")` | why a trait impl's associated values are what they are |
| `supersedes!(NEW, OLD, "...")` | this value replaced that one, and why |
| `rejected!(ITEM, "...")` | a value that was considered and turned down |
| `#[derive(Record)]` | reads a struct as named fields, for storage and transport |

Each expands to an item a compiler checks. Nothing here formats or prints: the macros exist so that
a reason has an identity, so a second reason for the same item is a duplicate the build can refuse
and a missing one is an absence the build can see.

On their own these macros only record. What enforces them is the checker in the
[Premise repository](https://github.com/Datamine-Crypto/premise), which refuses an item with no
reason, a reason written twice, and a reason that only restates the name of the thing it explains.

This crate depends on nothing. It is a proc-macro crate and can be dropped into any Rust workspace
whether or not the rest of Premise is there.

## More

The manual is the [repository README](https://github.com/Datamine-Crypto/premise). It covers the
four laws, every check, and a worked example end to end.

MIT licensed.
