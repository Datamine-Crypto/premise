use crate::hex::{bytes_of, hex_text, prefixed, unprefixed};
use patterns_macros::{because, source};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    Nothing,
    Flag(bool),
    Whole(i64),
    Real(f64),
    Text(String),
    List(Vec<Field>),
    Table(Vec<(String, Field)>),
}
because!(Field, "the one shape every stored or sent value takes: nothing, a flag, a whole number, a real, a text, a list or a table of named fields, so a store or a wire writes each once and never sees the type behind it");

pub trait Fielded: Sized {
    fn field(&self) -> Field;
    fn refielded(field: &Field) -> Option<Self>;
}
because!(Fielded, "a value read as a field and a field read back as the value, the two directions a record crosses between a program and what keeps it");

pub fn table_of(field: &Field) -> Option<&[(String, Field)]> {
    match field {
        Field::Table(rows) => Some(rows),
        _ => None,
    }
}
because!(table_of, "the named fields under a table field, or nothing when the field is not a table, the first step of reading any record back");

pub fn looked_field<'a>(table: &'a [(String, Field)], name: &str) -> &'a Field {
    let camel = camel_of(name);
    table
        .iter()
        .find(|(key, _)| key == name || *key == camel)
        .map(|(_, field)| field)
        .unwrap_or(&Field::Nothing)
}
because!(looked_field, "one named field out of a table, or nothing when the table lacks it, so an absent column reads as an absent value rather than a failure");

pub fn text_of_field(field: &Field) -> Option<&str> {
    match field {
        Field::Text(text) => Some(text),
        _ => None,
    }
}
because!(text_of_field, "the text under a text field, or nothing, the reading a vocabulary value is restored through");

fn whole_of(field: &Field) -> Option<i64> {
    match field {
        Field::Whole(value) => Some(*value),
        Field::Real(value) => Some(*value as i64),
        Field::Text(text) => text.parse().ok(),
        Field::Flag(flag) => Some(i64::from(*flag)),
        _ => None,
    }
}

fn real_of(field: &Field) -> Option<f64> {
    match field {
        Field::Real(value) => Some(*value),
        Field::Whole(value) => Some(*value as f64),
        Field::Text(text) => text.parse().ok(),
        _ => None,
    }
}

impl Fielded for bool {
    fn field(&self) -> Field {
        Field::Flag(*self)
    }

    fn refielded(field: &Field) -> Option<bool> {
        match field {
            Field::Flag(flag) => Some(*flag),
            other => whole_of(other).map(|value| value != 0),
        }
    }
}

impl Fielded for i64 {
    fn field(&self) -> Field {
        Field::Whole(*self)
    }

    fn refielded(field: &Field) -> Option<i64> {
        whole_of(field)
    }
}

impl Fielded for u64 {
    fn field(&self) -> Field {
        Field::Whole(i64::try_from(*self).unwrap_or(i64::MAX))
    }

    fn refielded(field: &Field) -> Option<u64> {
        whole_of(field).and_then(|value| u64::try_from(value).ok())
    }
}

impl Fielded for u32 {
    fn field(&self) -> Field {
        Field::Whole(i64::from(*self))
    }

    fn refielded(field: &Field) -> Option<u32> {
        u64::refielded(field)?.try_into().ok()
    }
}

impl Fielded for usize {
    fn field(&self) -> Field {
        u64::field(&(*self as u64))
    }

    fn refielded(field: &Field) -> Option<usize> {
        u64::refielded(field).map(|value| value as usize)
    }
}

impl Fielded for u128 {
    fn field(&self) -> Field {
        Field::Text(self.to_string())
    }

    fn refielded(field: &Field) -> Option<u128> {
        match field {
            Field::Text(text) => text.parse().ok(),
            other => whole_of(other).and_then(|value| u128::try_from(value).ok()),
        }
    }
}

impl Fielded for i128 {
    fn field(&self) -> Field {
        match *self < 0 {
            true => Field::Text(self.to_string()),
            false => u128::field(&self.unsigned_abs()),
        }
    }

    fn refielded(field: &Field) -> Option<i128> {
        text_of_field(field)
            .and_then(|text| text.parse().ok())
            .or_else(|| whole_of(field).map(i128::from))
    }
}

impl Fielded for f64 {
    fn field(&self) -> Field {
        Field::Real(*self)
    }

    fn refielded(field: &Field) -> Option<f64> {
        real_of(field)
    }
}

impl Fielded for String {
    fn field(&self) -> Field {
        Field::Text(self.clone())
    }

    fn refielded(field: &Field) -> Option<String> {
        match field {
            Field::Text(text) => Some(text.clone()),
            Field::Whole(value) => Some(value.to_string()),
            Field::Real(value) => Some(value.to_string()),
            _ => None,
        }
    }
}

impl<T: Fielded> Fielded for Option<T> {
    fn field(&self) -> Field {
        match self {
            Some(value) => value.field(),
            None => Field::Nothing,
        }
    }

    fn refielded(field: &Field) -> Option<Option<T>> {
        match field {
            Field::Nothing => Some(None),
            other => T::refielded(other).map(Some),
        }
    }
}

impl<T: Fielded> Fielded for Vec<T> {
    fn field(&self) -> Field {
        Field::List(self.iter().map(Fielded::field).collect())
    }

    fn refielded(field: &Field) -> Option<Vec<T>> {
        match field {
            Field::List(items) => items.iter().map(T::refielded).collect(),
            Field::Nothing => Some(vec![]),
            _ => None,
        }
    }
}

impl<A: Fielded, B: Fielded> Fielded for (A, B) {
    fn field(&self) -> Field {
        Field::List(vec![self.0.field(), self.1.field()])
    }

    fn refielded(field: &Field) -> Option<(A, B)> {
        match field {
            Field::List(items) => match items.as_slice() {
                [first, second] => Some((A::refielded(first)?, B::refielded(second)?)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl<V: Fielded> Fielded for BTreeMap<String, V> {
    fn field(&self) -> Field {
        Field::Table(self.iter().map(|(key, value)| (key.clone(), value.field())).collect())
    }

    fn refielded(field: &Field) -> Option<BTreeMap<String, V>> {
        let rows = table_of(field)?;
        rows.iter()
            .map(|(key, value)| V::refielded(value).map(|v| (key.clone(), v)))
            .collect()
    }
}

impl<const N: usize> Fielded for [u8; N] {
    fn field(&self) -> Field {
        Field::Text(prefixed(&hex_text(self)))
    }

    fn refielded(field: &Field) -> Option<[u8; N]> {
        bytes_of(unprefixed(text_of_field(field)?))?.try_into().ok()
    }
}

pub fn recorded<T: Fielded>(value: &T) -> Field {
    value.field()
}
because!(recorded, "a value as its field, the call a binding makes where the trait method is not in reach");

pub fn restored<T: Fielded>(field: &Field) -> Option<T> {
    T::refielded(field)
}
because!(restored, "a value read back from its field, the reverse of recorded");

pub fn restored_all<T: Fielded>(fields: &[Field]) -> Vec<T> {
    fields.iter().filter_map(T::refielded).collect()
}
because!(restored_all, "every value that reads back whole out of a list of fields, with the rows that do not fit the shape left out rather than failing the whole read");

pub struct WireNaming;
source!(WireNaming, "the convention the retired dashboard's wire and its stores follow: a program writes a field name with underscores between its words and the wire writes the same name with each later word capitalised instead");

const JOINER: char = '_';
because!(JOINER, WireNaming, "the mark between the words of a field name as a program writes it, which the wire drops");

pub fn camel_of(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut lift = false;
    for c in name.chars() {
        if c == JOINER {
            lift = true;
        } else if lift {
            out.extend(c.to_uppercase());
            lift = false;
        } else {
            out.push(c);
        }
    }
    out
}
because!(camel_of, "a field name as a wire writes it, the words run together with each one after the first capitalised, so a record read off a wire finds its fields under either spelling");
