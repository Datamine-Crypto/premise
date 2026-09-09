# premise_web3

[![crates.io](https://img.shields.io/crates/v/premise_web3.svg)](https://crates.io/crates/premise_web3)
[![docs.rs](https://img.shields.io/docsrs/premise_web3)](https://docs.rs/premise_web3)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Datamine-Crypto/premise/blob/main/LICENSE)

**Reading an EVM chain.** Addresses that carry their checksum, keccak digests, event logs decoded
from a declared layout, and the pool arithmetic a price comes out of.

```toml
[dependencies]
premise_web3 = "0.1"
```

No provider, no transport, no async runtime. This crate takes logs you already have and turns them
into typed values. How you fetched them is your business.

## What is in it

| module | holds |
|---|---|
| `hex` | `Address` as a packed `(u128, u32)`, EIP-55 checksumming, text in and out, short labels |
| `abi` | `Shape` and `Built`: declare an event's layout once, get its signature, topic and decoder |
| `keccak` | `hashed`, the digest everything above is built on |
| `dex` | virtual reserves from liquidity and a square-root price, reserve prices, daily prices |

## Decoding is declared, not written

An event is a `Shape`: a name and a layout of typed words, some indexed and some not. From that one
declaration the crate derives the signature string, the topic hash, and the decoder, so a log is
read by the same description that computed the topic it was matched on. There is no second place
where field order is written down and no way for the two to disagree.

```rust
use premise_web3::abi::{Table, decoded_all};

let known = Table::from(&shapes);
let (events, skipped) = decoded_all(&known, &logs);
```

`complete_blocks`, `after_block`, `first_block` and `starts_behind` handle the part everybody gets
wrong: a log page that ends mid-block, and a cursor that has fallen behind the range you were given.

`Address` implements `Fielded` from [`premise`](https://crates.io/crates/premise), so it stores and
transports like any other value without a bespoke serialiser.

## More

The manual is the [repository README](https://github.com/Datamine-Crypto/premise). Every public
item here carries a `because!` saying why it exists, so the source is where you learn what a
function is for.

MIT licensed.
