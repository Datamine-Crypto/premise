use crate::keccak::hashed;
use patterns::record::{text_of_field, Field, Fielded};
use patterns_macros::{because, source};

use patterns::hex::{bytes_of, hex_text, prefixed};

pub struct Eip55;
source!(Eip55, "the checksum spelling of an address that wallets show, which capitalises a digit when the matching nibble of the hash of the lowercase text is high");

pub struct AddressWidth;
source!(AddressWidth, "the width of an account address on the chain and the two integer parts this library splits it into so a spec can state one as numbers");

pub const ADDRESS_BYTES: usize = 20;
because!(ADDRESS_BYTES, AddressWidth, "the bytes an account address occupies on the chain, fewer than a word, which is why an address sits in the low end of the word that carries it");

const HIGH_PART_BYTES: usize = 16;
because!(HIGH_PART_BYTES, AddressWidth, "the bytes of an address the largest primitive integer can hold, so the address is split into that part and a short tail rather than into twenty separate numbers");

const LOW_PART_BYTES: usize = ADDRESS_BYTES - HIGH_PART_BYTES;

const HIGH_NIBBLE: u8 = 8;
because!(HIGH_NIBBLE, Eip55, "the nibble value from which the checksum capitalises the letter at that position, the top half of the sixteen values a nibble takes");

const NIBBLES_PER_BYTE: usize = 2;
because!(NIBBLES_PER_BYTE, Eip55, "the hex digits one byte of the hash answers for, the high nibble first, which is how a digit position maps onto the hash");

const LOW_NIBBLE_MASK: u8 = 15;
because!(LOW_NIBBLE_MASK, Eip55, "the bits of a byte that hold its low nibble, every bit of the lower half set, which keeps the low digit once the high one has been shifted away");

const NIBBLE_BITS: u32 = 4;
because!(NIBBLE_BITS, Eip55, "the bits in one nibble, the shift that brings a byte's high nibble down to a value the threshold can be compared with");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Address {
    pub bytes: [u8; ADDRESS_BYTES],
}
because!(Address, AddressWidth, "an account on the chain as the bytes the chain uses, so equality is on bytes and never on the case of a spelling");

pub const fn address(hi: u128, lo: u32) -> Address {
    let high = hi.to_be_bytes();
    let low = lo.to_be_bytes();
    let mut bytes = [0u8; ADDRESS_BYTES];
    let mut at = 0;
    while at < HIGH_PART_BYTES {
        bytes[at] = high[at];
        at += 1;
    }
    while at < ADDRESS_BYTES {
        bytes[at] = low[at - HIGH_PART_BYTES];
        at += 1;
    }
    Address { bytes }
}
because!(address, "an address assembled from the two integers a spec states it as, constant so a spec can declare the assembled value as a derivation of the two parts");

pub fn address_parts(value: &Address) -> (u128, u32) {
    let mut high = [0u8; HIGH_PART_BYTES];
    let mut low = [0u8; LOW_PART_BYTES];
    high.copy_from_slice(&value.bytes[..HIGH_PART_BYTES]);
    low.copy_from_slice(&value.bytes[HIGH_PART_BYTES..]);
    (u128::from_be_bytes(high), u32::from_be_bytes(low))
}
because!(address_parts, "the reverse of address, so a test or a tool can state what it read in the same two numbers a spec declares");

pub const ZERO_ADDRESS: Address = address(0, 0);

pub fn address_text(value: &Address) -> String {
    prefixed(&hex_text(&value.bytes))
}
because!(address_text, "an address in the lowercase prefixed form every store and every request uses as its key");

pub fn address_from_text(text: &str) -> Option<Address> {
    let bytes = bytes_of(text)?;
    let mut out = [0u8; ADDRESS_BYTES];
    match bytes.len() == ADDRESS_BYTES {
        true => {
            out.copy_from_slice(&bytes);
            Some(Address { bytes: out })
        }
        false => None,
    }
}
because!(address_from_text, "an address parsed from text of exactly its width, so a request naming an account of the wrong length is refused rather than padded");

pub fn checksummed(value: &Address) -> String {
    let lower = hex_text(&value.bytes);
    let digest = hashed(lower.as_bytes());
    let mut out = String::new();
    for (at, c) in lower.chars().enumerate() {
        let byte = digest[at / NIBBLES_PER_BYTE];
        let nibble = match at % NIBBLES_PER_BYTE == 0 {
            true => byte >> NIBBLE_BITS,
            false => byte & LOW_NIBBLE_MASK,
        };
        match nibble >= HIGH_NIBBLE {
            true => out.push(c.to_ascii_uppercase()),
            false => out.push(c),
        }
    }
    prefixed(&out)
}
because!(checksummed, "an address in the mixed case spelling a wallet shows, computed for display only, since every store keys on the lowercase form and a checksum that leaked into a key would split one account in two");

pub fn short_label(value: &Address, chars: usize) -> String {
    let text = address_text(value);
    let head: String = text.chars().take(chars).collect();
    let mut out = head;
    out.push_str(&ELLIPSIS.iter().collect::<String>());
    out
}
because!(short_label, "the first few characters of an address followed by an ellipsis, the label a chart bar gets when the whole address would not fit under it");

const ELLIPSIS: [char; ELLIPSIS_DOTS] = ['.', '.', '.'];

const ELLIPSIS_DOTS: usize = 3;
because!(ELLIPSIS_DOTS, AddressWidth, "the dots that stand for the rest of a shortened address, enough that a reader takes them for neither a sentence end nor a typo");

impl Fielded for Address {
    fn field(&self) -> Field {
        Field::Text(address_text(self))
    }

    fn refielded(field: &Field) -> Option<Address> {
        address_from_text(text_of_field(field)?)
    }
}
