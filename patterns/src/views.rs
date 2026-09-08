use crate::money::rounded;
use crate::render::{filled, Named, Piece};
use patterns_macros::because;

pub fn widened_usize(value: usize) -> u64 {
    u64::try_from(value).unwrap_or_default()
}
because!(widened_usize, "a length or an index as the number a page answers, the widening a binding may not write as a cast");

pub fn named_from<T: Named + Copy>(items: &[T], text: &str) -> Option<T> {
    items.iter().copied().find(|item| item.text() == text)
}
because!(named_from, "the value of a vocabulary whose text is the one given, or nothing, which is how a request parameter becomes a value without a match in a binding");

pub fn label_of_keyed<K: PartialEq, T: Copy>(items: &[T], key_of: fn(&T) -> K, key: K, label: fn(&T) -> String, fallback: &str) -> String {
    items
        .iter()
        .find(|item| key_of(item) == key)
        .map(label)
        .unwrap_or_else(|| String::from(fallback))
}
because!(label_of_keyed, "the label of the vocabulary entry whose key matches, or a fallback word, which is how a pool address becomes the venue name the feed shows");

pub fn sum_seconds<A>(pairs: &[(A, u64)]) -> u64 {
    pairs.iter().map(|(_, n)| *n).fold(0u64, |a, b| a.saturating_add(b))
}
because!(sum_seconds, "the sum of the counts in a list of pairs, the total a feed shows beside its all entry");

pub fn sum_thirds<A, B>(triples: &[(A, B, u64)]) -> u64 {
    triples.iter().map(|(_, _, n)| *n).fold(0u64, |a, b| a.saturating_add(b))
}
because!(sum_thirds, "the sum of the counts in a list of triples, the same total read from a filter menu that already carries labels");

pub fn options_with_counts<K: Copy + PartialEq>(kinds: &[K], totals: &[(K, u64)], label: fn(&K) -> String) -> Vec<(K, String, u64)> {
    kinds
        .iter()
        .map(|kind| {
            let count = totals.iter().find(|(k, _)| k == kind).map(|(_, n)| *n).unwrap_or(0);
            (*kind, label(kind), count)
        })
        .filter(|(_, _, count)| *count > 0)
        .collect()
}
because!(options_with_counts, "a filter menu: every kind the menu offers with its label and how many rows it has, with the empty kinds left out, in the order the spec lists them");

pub fn faced<A, P: Named>(items: &[A], pieces_of: fn(&A) -> &'static [Piece<P>], first: &str, second: &str, third: &str, fourth: &str) -> Vec<String> {
    items
        .iter()
        .map(|item| filled(pieces_of(item), first, second, third, fourth))
        .collect()
}
because!(faced, "the text of every item filled from the pieces a spec table gives it, so a binding renders a whole menu of labels with one call");

pub fn picked_names(flags: &[bool], yes: &str, no: &str) -> Vec<String> {
    flags
        .iter()
        .map(|flag| match flag {
            true => String::from(yes),
            false => String::from(no),
        })
        .collect()
}
because!(picked_names, "one of two names for each flag, the token name each feed row is denominated in");

pub fn converted_maybe<A, B: From<A>>(items: Option<Vec<A>>) -> Option<Vec<B>> {
    items.map(|list| list.into_iter().map(B::from).collect())
}
because!(converted_maybe, "an optional list converted when present, the optional twin of converted");

pub fn facet_of<F, T: crate::seq::Facet<F>>(item: &T) -> T::Value {
    item.facet()
}
because!(facet_of, "one projection of one item, the single form of a facet, so a binding hands a projection to a list pattern as a plain function");

pub fn counted_rounded(value: f64, places: u32) -> f64 {
    rounded(value, places)
}
because!(counted_rounded, "a figure rounded to places, kept here beside the view helpers so the views module reads whole; the same rounding as everywhere else");

pub fn listed_maybe<T: Clone>(value: &Option<T>) -> Option<T> {
    value.clone()
}
because!(listed_maybe, "a borrowed maybe as an owned one, so a binding hands an optional row on without a method call");

pub fn far_future() -> u64 {
    !0u64
}
because!(far_future, "the largest moment, every bit set, the upper edge of a date range a reader left open");

pub fn keep_in<A: Clone, B: Copy + PartialEq>(items: &[A], read: fn(&A) -> B, allowed: &[B]) -> Vec<A> {
    items
        .iter()
        .filter(|item| allowed.contains(&read(item)))
        .cloned()
        .collect()
}
because!(keep_in, "the items whose projection is one of the allowed values, for a table that shows some columns only on some deployments");

pub fn looked_up<K: Copy + PartialEq, V: Copy>(keys: &[K], table: &[(K, V)], fallback: V) -> Vec<V> {
    keys.iter()
        .map(|key| table.iter().find(|(k, _)| k == key).map(|(_, v)| *v).unwrap_or(fallback))
        .collect()
}
because!(looked_up, "one value per key out of a small table of pairs, or a fallback where the table has no entry, the counts a menu shows beside its entries");

pub fn converted_each<A, B: From<A>>(lists: Vec<Vec<A>>) -> Vec<Vec<B>> {
    lists
        .into_iter()
        .map(|list| list.into_iter().map(B::from).collect())
        .collect()
}
because!(converted_each, "a list of lists converted through the conversion a spec declares, the nested twin of converted");

pub fn text_of(text: &str) -> String {
    String::from(text)
}
because!(text_of, "a borrowed text as an owned one, the copy a binding may not make with a method");

pub fn described<A, B, P: Named + 'static>(items: &[A], pieces_of: fn(&A) -> &'static [Piece<P>], word_of: fn(&A) -> fn(&B) -> &'static [Piece<P>], arg: &B, first: &str, second: &str, third: &str, blank: &str) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            let fourth = filled(word_of(item)(arg), first, second, third, blank);
            filled(pieces_of(item), first, second, third, &fourth)
        })
        .collect()
}
because!(described, "the text of every item where the fourth hole is itself a phrase chosen per item from a table keyed by something else, a badge's description naming the venue its deployment trades on");
