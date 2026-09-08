use patterns_macros::{because, source};

pub trait Named {
    fn text(&self) -> &'static str;
}

pub fn named<T: Named>(value: &T) -> &'static str {
    value.text()
}

pub fn named_text<T: Named>(value: &T) -> String {
    String::from(value.text())
}
because!(named_text, "the display text of a named value as an owned string, for a page that answers the text rather than lending it");

pub fn digits<T: std::fmt::Display>(value: T) -> String {
    format!("{}", value)
}

pub fn label(name: &str, value: &str) -> String {
    format!("{}={}", name, value)
}

pub fn joined(parts: &[String], between: &str) -> String {
    parts.join(between)
}

pub fn numbered(stem: &str, at: u32, places: usize) -> String {
    format!("{}{:0>width$}", stem, at, width = places)
}
because!(named, "the display text for a type that has one, so app never writes a user-facing string");
because!(digits, "a number as text, kept separate from named because a number has no vocabulary to name it");
because!(label, "a name and its value joined, the shape almost every rendered line takes");
because!(numbered, "a name with a number padded to a fixed width, so a run of saved files sorts in the order it was made rather than by the first digit of the count");
because!(joined, "several rendered parts into one line, so a separator is stated once here rather than at every call");

pub trait Shown<M> {
    fn shown(&self) -> &'static str;
}

pub fn shown_as<M, T: Shown<M>>(value: &T) -> &'static str {
    value.shown()
}
because!(Shown, "a second and further display text on a type that already has a Named one, told apart by a marker, so a vocabulary can carry a key, a label and a description without three enums");
because!(shown_as, "the text a marker selects on a value, the way value_of reads a marker's table, so app never writes a user-facing string for any of them");

pub trait Texted<M> {
    fn stem(&self) -> &'static str;
    fn number(&self) -> Option<u64>;
    fn tail(&self) -> &'static str;
}

pub fn texted<M, T: Texted<M>>(value: &T) -> String {
    let mut out = String::from(value.stem());
    if let Some(number) = value.number() {
        out.push_str(&digits(number));
    }
    out.push_str(value.tail());
    out
}
because!(Texted, "a display text that carries a number, split into the words before it, the constant, and the words after, so a spec never writes a digit into a string and the number is the one the constant states");
because!(texted, "the three parts of a Texted value joined into the text a reader sees, the only place a stem and its number meet");

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece<P> {
    Say(P),
    First,
    Second,
    Third,
    Fourth,
    Number(u64),
    Wide(u128),
    Grouped(u64),
    Fixed(f64, usize),
}
because!(Piece, "one part of a text with holes: a phrase the spec names, one of four values filled in at render time, or a number read from a constant, so no sentence with a figure in it is ever spelled out");

pub fn filled<P: Named>(pieces: &[Piece<P>], first: &str, second: &str, third: &str, fourth: &str) -> String {
    let mut out = String::new();
    for piece in pieces {
        match piece {
            Piece::Say(phrase) => out.push_str(phrase.text()),
            Piece::First => out.push_str(first),
            Piece::Second => out.push_str(second),
            Piece::Third => out.push_str(third),
            Piece::Fourth => out.push_str(fourth),
            Piece::Number(n) => out.push_str(&digits(n)),
            Piece::Wide(n) => out.push_str(&digits(n)),
            Piece::Grouped(n) => out.push_str(&grouped(*n)),
            Piece::Fixed(value, places) => out.push_str(&fixed(*value, *places)),
        }
    }
    out
}
because!(filled, "a text with holes rendered, the phrases in order with the four values and the numbers in their places, the one way a label with a token name or a figure in it is made");

pub fn fixed(value: f64, places: usize) -> String {
    format!("{:.*}", places, value)
}
because!(fixed, "a number with a fixed count of places after the point, the form a multiplier or a percent is shown in");

const GROUP_MARK: u8 = 44;
because!(GROUP_MARK, ThousandsGrouping, "the comma set between groups, written as its code point because a character literal is a value from nowhere in this zone");

const GROUP: usize = 3;
because!(GROUP, ThousandsGrouping, "the digits between separators when a large number is written for a reader");

pub struct ThousandsGrouping;
source!(ThousandsGrouping, "the convention of splitting a written number into groups of three digits from the right, which the dashboard's readers expect in every figure over a thousand");

pub fn grouped(value: u64) -> String {
    let plain = digits(value);
    let mut out = String::with_capacity(plain.len() + plain.len() / GROUP);
    for (at, c) in plain.chars().enumerate() {
        if at > 0 && (plain.len() - at) % GROUP == 0 {
            out.push(GROUP_MARK as char);
        }
        out.push(c);
    }
    out
}
because!(grouped, "a whole number with a separator every three digits from the right, the way a reader expects a large figure written");

pub trait Worded {
    fn text(&self) -> String;
    fn ink(&self) -> Option<u32>;
}
because!(Worded, "a run of a title as its text and the ink it is drawn in, read off a spec's title vocabulary so a chart title is assembled here without naming a token");

pub fn worded_text<T: Worded>(value: &T) -> String {
    value.text()
}
because!(worded_text, "the text of one title run");

pub fn worded_ink<T: Worded>(value: &T) -> Option<u32> {
    value.ink()
}
because!(worded_ink, "the ink of one title run, or none for a plain word");
