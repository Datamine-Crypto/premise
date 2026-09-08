use crate::arith::{mul, raise_by};
use crate::clamp::clamp_upper;
use crate::seq::{with, Keyed};
use patterns_macros::because;

pub fn dequeued<T: Clone>(items: &[T]) -> Option<(T, Vec<T>)> {
    items
        .split_first()
        .map(|(head, rest)| (head.clone(), rest.to_vec()))
}
because!(dequeued, "taking the front of a queue and the remainder together, so a caller never indexes zero and the empty queue is a value rather than a panic");

pub fn rotated<T: Clone>(items: &[T]) -> Vec<T> {
    match items.split_first() {
        Some((head, rest)) => with(rest, head.clone()),
        None => Vec::new(),
    }
}
because!(rotated, "one turn of a round robin, so a rota or a fair share of bays advances without a caller tracking an index that must wrap");

pub fn moved<T: Keyed + Clone>(from: &[T], to: &[T], key: T::Key) -> (Vec<T>, Vec<T>) {
    let mut left = Vec::new();
    let mut arrived = to.to_vec();
    for item in from {
        if item.key() == key {
            arrived.push(item.clone());
        } else {
            left.push(item.clone());
        }
    }
    (left, arrived)
}
because!(moved, "a transfer between two shelves as one step, so the element cannot be dropped from one without arriving on the other, which without and with as separate calls allow; every element under the key moves, so two sharing a key arrive together rather than one being stranded");

pub fn replaced<T: Keyed + Clone>(items: &[T], item: T) -> Vec<T> {
    let mut out = Vec::new();
    for held in items {
        if held.key() == item.key() {
            out.push(item.clone());
        } else {
            out.push(held.clone());
        }
    }
    out
}
because!(replaced, "an update in place that keeps the position, because with after without sends a queued element to the back and silently changes its turn; every element under the key is replaced, so two sharing a key both become the new element and a duplicate is not silently kept");

pub fn page<T: Clone>(items: &[T], size: usize, number: usize) -> Vec<T> {
    let start = clamp_upper(mul(size, number), items.len());
    let end = clamp_upper(raise_by(start, size), items.len());
    items[start..end].to_vec()
}
because!(page, "one page of a listing, with the arithmetic that ends a list saturated and clamped here so a request past the end is an empty page rather than a panic");
