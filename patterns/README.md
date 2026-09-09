# premise

[![crates.io](https://img.shields.io/crates/v/premise.svg)](https://crates.io/crates/premise)
[![docs.rs](https://img.shields.io/docsrs/premise)](https://docs.rs/premise)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Datamine-Crypto/premise/blob/main/LICENSE)

**The core pattern library.** Generic, parameterised logic that names no project noun and carries no
loose number, so the same function serves every project that reaches for it.

```toml
[dependencies]
premise = "0.1"
```

The crate is `premise` on the registry and `patterns` in code, because a pattern is what it holds:

```rust
use patterns::seq::{map, keep, fold};
use patterns::money::whole_tokens;
```

## What is in it

Thirty modules. The ones you reach for first:

| module | holds |
|---|---|
| `seq`, `lists` | map, keep, fold, paging, windows over slices, taking `fn` pointers rather than closures |
| `table` | lookups and insertions over `BTreeMap`, and `Shift` for a keyed mutation |
| `record` | `Field` and the `Fielded` trait: a struct read as named values, for storage and transport |
| `money` | whole tokens from wei, decimal notation, half-up rounding |
| `day`, `window` | days from timestamps, spans, and the arithmetic between two dates |
| `text` | day text, ISO day text, and the conventions a reader expects |
| `series`, `readings`, `charts` | points, lines and the shapes a chart is drawn from |
| `queue`, `order`, `select`, `bound`, `clamp` | ordering, choosing and holding a value inside a range |
| `arith`, `sign`, `chance`, `hex`, `levels` | the small numeric pieces the rest are built on |
| `rationale` | re-exports `because!`, `source!` and the other reason macros |

Every public function carries a `because!` saying why it exists, so `cargo doc` is not where you
learn what a function is for: the source is.

## The rule this library exists for

Premise's first law is that all logic lives in a pattern. A pattern names no project noun and holds
no literal but `0` and `1`, so it cannot know anything about your domain, and that is what makes it
reusable. A value that varies at runtime arrives as an argument; a thing a pattern needs from an
element arrives through a trait you implement. That is why these functions take `fn` pointers rather
than closures: a closure captures, and a captured value is a fact that escaped the spec.

## More

The manual is the [repository README](https://github.com/Datamine-Crypto/premise). It covers the
four laws, every check, and a worked example end to end. `cargo run -q --bin catalog` in that
repository prints every function and trait here with its signature, read from the source so it
cannot go stale.

`premise_macros` arrives with this crate. Add
[`premise_web3`](https://crates.io/crates/premise_web3) or
[`premise_cloudflare`](https://crates.io/crates/premise_cloudflare) only if you want a chain or a
Worker.

MIT licensed.
