use patterns_macros::{because, source};

pub struct HexNotation;
source!(HexNotation, "the way the chain's tooling writes bytes as text: a zero and an x, then two lowercase digits for each byte");

const PREFIX_ZERO: u8 = 48;
because!(PREFIX_ZERO, HexNotation, "the digit zero that opens hex text, written as its code point because a string holding it would start a word with a digit and state a fact in prose");

const PREFIX_X: u8 = 120;
because!(PREFIX_X, HexNotation, "the letter x that follows the zero in hex text, written as its code point beside the zero so the two are declared as the pair they are");

pub fn hex_text(bytes: &[u8]) -> String {
    ::hex::encode(bytes)
}
because!(hex_text, "bytes as lowercase hex digits with no prefix, the form the stores key on, so a key never differs from itself by case");

const OPENER_CHARS: usize = 2;
because!(OPENER_CHARS, HexNotation, "the two characters of the opener, a zero and an x, that prefixed adds and unprefixed removes");

pub fn prefixed(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + OPENER_CHARS);
    out.push(PREFIX_ZERO as char);
    out.push(PREFIX_X as char);
    out.push_str(text);
    out
}
because!(prefixed, "hex text with the opening pair the chain's tooling expects in front of it, added here so no other file spells the pair");

pub fn unprefixed(text: &str) -> &str {
    let opener = [PREFIX_ZERO as char, PREFIX_X as char];
    let lower = text.get(..OPENER_CHARS).map(|s| s.to_ascii_lowercase());
    match lower {
        Some(head) if head.chars().eq(opener) => &text[OPENER_CHARS..],
        _ => text,
    }
}
because!(unprefixed, "hex text with the opening pair removed when it is there, so a value read from the chain or typed by a person parses the same either way");

pub fn bytes_of(text: &str) -> Option<Vec<u8>> {
    ::hex::decode(unprefixed(text)).ok()
}
because!(bytes_of, "hex text back into bytes, with or without its opener, answering nothing rather than failing on text that is not hex");
