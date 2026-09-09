# premise_cloudflare

[![crates.io](https://img.shields.io/crates/v/premise_cloudflare.svg)](https://crates.io/crates/premise_cloudflare)
[![docs.rs](https://img.shields.io/docsrs/premise_cloudflare)](https://docs.rs/premise_cloudflare)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Datamine-Crypto/premise/blob/main/LICENSE)

**The Worker side.** Records in D1, compressed objects in R2, JSON field shapes, and the HTTP edge
pieces a Worker binds its own policy to.

```toml
[dependencies]
premise_cloudflare = "0.1"
```

Built on the [`worker`](https://crates.io/crates/worker) crate. It adds the parts every Worker
writes again: turning a typed record into rows and back, gzip on the way into a bucket, and the
header and authorisation work that is easy to get subtly wrong.

## What is in it

| module | holds |
|---|---|
| `d1` | schema statements, upserts and reads for anything implementing `Fielded`, with column naming that dodges SQLite's reserved words |
| `r2` | `put_gzip` and `get_gzip`, plus listing keys under a prefix |
| `json` | a `Field` to and from `serde_json::Value`, so one record shape serves the database and the wire |
| `edge` | security headers, the Cloudflare request headers worth reading, and a constant-time admin check |

## One record shape, three destinations

A type implements `Fielded` once, from [`premise`](https://crates.io/crates/premise). D1 columns,
R2 payloads and JSON responses are all derived from it, so a field added to a struct reaches storage
and the wire without a second declaration in either.

```rust
use premise_cloudflare::d1::upsert_statements;
use premise_cloudflare::json::json_text_of;
```

## The admin check

`edge` compares a request's bearer token against the configured secret over a fixed number of bytes,
so neither the position where they differ nor the length of either one changes how long the answer
takes. A secret longer than the walk is refused on length rather than read short. That is the kind
of decision this library exists to make once, with the reason attached, rather than in every Worker.

## More

The manual is the [repository README](https://github.com/Datamine-Crypto/premise). Every public
item here carries a `because!` saying why it exists.

MIT licensed.
