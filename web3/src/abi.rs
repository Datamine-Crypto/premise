use crate::hex::{Address, ADDRESS_BYTES, ZERO_ADDRESS};
use crate::keccak::hashed;
use patterns::render::{texted, Texted};
use patterns_macros::{because, source};
use ruint::aliases::U256;
use std::collections::VecDeque;

pub struct AbiEncoding;
source!(AbiEncoding, "the application binary interface of the chain, which lays an event out as a hashed signature in the first topic, indexed values in the topics after it, and the rest in words of data with dynamic bytes reached through an offset");

pub const WORD_BYTES: usize = 32;
because!(WORD_BYTES, AbiEncoding, "the bytes in one encoded word, the unit every static value is padded to and every offset is measured in");

const BITS_PER_BYTE: usize = 8;
because!(BITS_PER_BYTE, AbiEncoding, "the bits in a byte, needed to turn the word width in bytes into the bit position of the sign");

const WORD_BITS: usize = WORD_BYTES * BITS_PER_BYTE;

const SIGN_BIT: usize = WORD_BITS - 1;

const OPEN: u8 = 40;
because!(OPEN, AbiEncoding, "the opening parenthesis of a signature, written as its code point so the signature text is assembled from named parts rather than spelled");

const CLOSE: u8 = 41;
because!(CLOSE, AbiEncoding, "the closing parenthesis of a signature, the code point after the opening one");

const BETWEEN: u8 = 44;
because!(BETWEEN, AbiEncoding, "the comma between two parameter types in a signature, with no space after it, which is the one spelling the chain hashes");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Address,
    Unsigned,
    Signed,
    Bytes,
}
because!(Kind, AbiEncoding, "the four ways a word is read: the low bytes as an account, the whole as an unsigned or a two's complement signed number, or as an offset to a run of bytes; a width narrower than a word changes nothing about the reading");

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Word {
    Address(Address),
    Unsigned(U256),
    Signed(i128),
    Bytes(Vec<u8>),
}
because!(Word, AbiEncoding, "one decoded value, kept as wide as the encoding allows so a narrowing to the width a spec wants happens in one named step");

pub trait Kinded {
    fn kind(&self) -> Kind;
}
because!(Kinded, "how a spec's type vocabulary tells the decoder which reading a parameter takes, so the decoder never names a type of the chain");

pub struct Signature;
because!(Signature, "the marker under which a parameter type spells itself the way the interface hashes it, apart from any text a reader sees");

pub trait Shape {
    type Ty: Texted<Signature> + Kinded;
    fn stem(&self) -> &'static str;
    fn layout(&self) -> Vec<(Self::Ty, bool)>;
}
because!(Shape, "an event as the encoding sees it, a name and an ordered list of typed parameters each marked indexed or not, from which the signature, the topic and the reading of a log all follow");

pub trait Built: Sized {
    type Shape: Shape;
    fn built(shape: &Self::Shape, words: Words) -> Self;
}
because!(Built, "how a spec turns the decoded words of a known shape into its own typed event, reading the words in declaration order, so the decoder stays generic and the spec names its fields");

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Log {
    pub address: Address,
    pub block: u64,
    pub index: u64,
    pub transaction: [u8; WORD_BYTES],
    pub time: u64,
    pub topics: Vec<[u8; WORD_BYTES]>,
    pub data: Vec<u8>,
}
because!(Log, AbiEncoding, "one raw event as the chain explorer returns it, hex already parsed to bytes and numbers, the input of every decoding");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Place {
    pub contract: Address,
    pub block: u64,
    pub index: u64,
    pub transaction: [u8; WORD_BYTES],
    pub time: u64,
}
because!(Place, "where an event happened: the emitting program, the block and the position in it, the transaction and the block's time, carried beside every decoded event because every consumer orders and dates by it");

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Occurred<E> {
    pub at: Place,
    pub event: E,
}
because!(Occurred, "a decoded event with its place, the one shape every consumer of the chain reads");

pub struct Words {
    held: VecDeque<Word>,
}
because!(Words, "the decoded values of one log in declaration order, read from the front one at a time so a spec builds its event without naming positions");

pub fn signature<S: Shape>(shape: &S) -> String {
    let mut out = String::from(shape.stem());
    out.push(OPEN as char);
    let mut first = true;
    for (ty, _) in shape.layout() {
        match first {
            true => first = false,
            false => out.push(BETWEEN as char),
        }
        out.push_str(&texted::<Signature, S::Ty>(&ty));
    }
    out.push(CLOSE as char);
    out
}
because!(signature, "the canonical text of an event, its name and parameter types with no spaces and no names, which is what the chain hashes into the first topic");

pub fn topic<S: Shape>(shape: &S) -> [u8; WORD_BYTES] {
    let mut out = [0u8; WORD_BYTES];
    out.copy_from_slice(&hashed(signature(shape).as_bytes()));
    out
}
because!(topic, "the first topic a log of this shape carries, derived from the signature rather than stated, so no hash is ever written down and a changed parameter list changes the topic with it");

fn word_from(data: &[u8], from: usize) -> [u8; WORD_BYTES] {
    let mut out = [0u8; WORD_BYTES];
    let end = from.saturating_add(WORD_BYTES).min(data.len());
    let taken = data.get(from..end).unwrap_or(&[]);
    out[..taken.len()].copy_from_slice(taken);
    out
}

fn unsigned(word: &[u8; WORD_BYTES]) -> U256 {
    U256::from_be_bytes(*word)
}

fn signed(word: &[u8; WORD_BYTES]) -> i128 {
    let raw = unsigned(word);
    match raw.bit(SIGN_BIT) {
        true => {
            let magnitude: u128 = U256::ZERO.wrapping_sub(raw).saturating_to();
            i128::try_from(magnitude).map(|m| -m).unwrap_or(i128::MIN)
        }
        false => i128::try_from(raw.saturating_to::<u128>()).unwrap_or(i128::MAX),
    }
}

fn account(word: &[u8; WORD_BYTES]) -> Address {
    let mut bytes = [0u8; ADDRESS_BYTES];
    bytes.copy_from_slice(&word[WORD_BYTES - ADDRESS_BYTES..]);
    Address { bytes }
}

fn dynamic(data: &[u8], head: &[u8; WORD_BYTES]) -> Vec<u8> {
    let offset: usize = unsigned(head).saturating_to();
    let length: usize = unsigned(&word_from(data, offset)).saturating_to();
    let start = offset.saturating_add(WORD_BYTES);
    let end = start.saturating_add(length).min(data.len());
    data.get(start..end).unwrap_or(&[]).to_vec()
}

pub fn words(log: &Log, layout: &[(Kind, bool)]) -> Words {
    let mut held = VecDeque::with_capacity(layout.len());
    let mut topic_at = 1;
    let mut data_at = 0;
    for (kind, indexed) in layout {
        let word = match indexed {
            true => {
                let w = log.topics.get(topic_at).copied().unwrap_or([0u8; WORD_BYTES]);
                topic_at += 1;
                w
            }
            false => {
                let w = word_from(&log.data, data_at * WORD_BYTES);
                data_at += 1;
                w
            }
        };
        held.push_back(match kind {
            Kind::Address => Word::Address(account(&word)),
            Kind::Unsigned => Word::Unsigned(unsigned(&word)),
            Kind::Signed => Word::Signed(signed(&word)),
            Kind::Bytes => Word::Bytes(dynamic(&log.data, &word)),
        });
    }
    Words { held }
}
because!(words, "the values of one log read by a layout: indexed parameters take the topics after the first in order, the rest take data words in order, and a missing word reads as zero rather than failing, since the explorer sometimes returns an empty field for a zero");

pub fn address_then(words: Words) -> (Address, Words) {
    let mut rest = words;
    let out = match rest.held.pop_front() {
        Some(Word::Address(a)) => a,
        _ => ZERO_ADDRESS,
    };
    (out, rest)
}
because!(address_then, "the next value as an account, with the rest of the values handed back, so a spec reads its parameters one after another without positions");

pub fn amount_then(words: Words) -> (u128, Words) {
    let mut rest = words;
    let out = match rest.held.pop_front() {
        Some(Word::Unsigned(v)) => v.saturating_to(),
        _ => 0,
    };
    (out, rest)
}
because!(amount_then, "the next value as an unsigned amount narrowed to the widest primitive, pinned at its ceiling rather than wrapped, since no real balance approaches it and a wrapped one would read as small");

pub fn count_then(words: Words) -> (u64, Words) {
    let (amount, rest) = amount_then(words);
    (U256::from(amount).saturating_to(), rest)
}
because!(count_then, "the next value as a block number or a count, narrower than an amount because a block number never nears the width of a balance, so the two are told apart by type");

pub fn signed_then(words: Words) -> (i128, Words) {
    let mut rest = words;
    let out = match rest.held.pop_front() {
        Some(Word::Signed(v)) => v,
        _ => 0,
    };
    (out, rest)
}
because!(signed_then, "the next value as a signed number, for the deltas a pool swap reports, which a consumer may read as a direction");

pub fn bytes_then(words: Words) -> (Vec<u8>, Words) {
    let mut rest = words;
    let out = match rest.held.pop_front() {
        Some(Word::Bytes(v)) => v,
        _ => vec![],
    };
    (out, rest)
}
because!(bytes_then, "the next value as the run of bytes a dynamic parameter carries, empty when the log had none");

pub struct Table<S> {
    rows: Vec<([u8; WORD_BYTES], S)>,
}
because!(Table, "the topics of every known shape computed once, because hashing every signature again for every log would cost more than the decoding");

pub fn table<S: Shape + Clone>(shapes: &[S]) -> Table<S> {
    Table {
        rows: shapes.iter().map(|s| (topic(s), s.clone())).collect(),
    }
}
because!(table, "the lookup from a first topic to its shape, built from the shapes a spec lists so a shape it does not list is unknown rather than guessed");

fn kinds<S: Shape>(shape: &S) -> Vec<(Kind, bool)> {
    shape
        .layout()
        .iter()
        .map(|(t, indexed)| (t.kind(), *indexed))
        .collect()
}

pub fn decoded<E: Built>(known: &Table<E::Shape>, log: &Log) -> Option<Occurred<E>> {
    let first = log.topics.first()?;
    let (_, shape) = known.rows.iter().find(|(t, _)| t == first)?;
    let layout = kinds(shape);
    let event = E::built(shape, words(log, &layout));
    Some(Occurred {
        at: Place {
            contract: log.address,
            block: log.block,
            index: log.index,
            transaction: log.transaction,
            time: log.time,
        },
        event,
    })
}
because!(decoded, "one log as a typed event with its place, or nothing when its first topic names no known shape, since a program emits events the application never asked about and those are skipped rather than failed");

pub fn decoded_all<E: Built>(known: &Table<E::Shape>, logs: &[Log]) -> (Vec<Occurred<E>>, usize) {
    let mut out = Vec::with_capacity(logs.len());
    let mut unknown = 0;
    for log in logs {
        match decoded(known, log) {
            Some(one) => out.push(one),
            None => unknown += 1,
        }
    }
    (out, unknown)
}
because!(decoded_all, "a run of logs decoded in order with the count of the ones no shape matched, which a caller reports rather than ignores, so a new event on a watched program shows up as a rising count");

pub fn complete_blocks(logs: &[Log]) -> Vec<Log> {
    let last = match logs.last() {
        Some(log) => log.block,
        None => return vec![],
    };
    logs.iter().filter(|l| l.block != last).cloned().collect()
}
because!(complete_blocks, "a page of logs with its last block dropped, since a page cut at a limit may have split that block and a stored batch must hold whole blocks for a guard on block numbers to be sound");

pub fn first_block(logs: &[Log]) -> Option<u64> {
    logs.first().map(|l| l.block)
}
because!(first_block, "the block of the first log of a page, which a cursor is checked against before the page is accepted");

pub fn last_block_or(logs: &[Log], fallback: u64) -> u64 {
    logs.last().map(|l| l.block).unwrap_or(fallback)
}
because!(last_block_or, "the block of the last log of a page, or a fallback when the page is empty, which is where a cursor moves to once the page is stored");

pub fn starts_behind(logs: &[Log], cursor: u64) -> bool {
    first_block(logs).map(|b| b <= cursor).unwrap_or(false)
}
because!(starts_behind, "whether a page begins at or before a cursor, which a sync refuses because storing it again would put one block in two batches");

pub fn after_block(logs: &[Log], cursor: u64) -> Vec<Log> {
    logs.iter().filter(|l| l.block > cursor).cloned().collect()
}
because!(after_block, "the logs of a batch past a cursor, the filter every replay applies so a batch delivered twice applies once");
