use crate::seq::Shift;
use patterns_macros::because;
use std::collections::BTreeMap;

pub fn looked<K: Ord, V: Clone>(map: &BTreeMap<K, V>, key: &K) -> Option<V> {
    map.get(key).cloned()
}
because!(looked, "one entry of a keyed table by its key, cloned, so a reducer reads a row the way found reads a list and never indexes");

pub fn placed_in<K: Ord, V>(map: BTreeMap<K, V>, key: K, value: V) -> BTreeMap<K, V> {
    let mut out = map;
    out.insert(key, value);
    out
}
because!(placed_in, "a keyed table with one entry set, taking the table by value and handing it back, so a reducer that owns its state updates a row without a copy of every other row");

pub fn shifted_in<M, K: Ord, V: Shift<M> + Clone>(map: BTreeMap<K, V>, key: K, fresh: V, by: V::By) -> BTreeMap<K, V> {
    let mut out = map;
    let current = out.remove(&key).unwrap_or(fresh);
    out.insert(key, current.shifted(by));
    out
}
because!(shifted_in, "one row of a keyed table moved by a step, starting from a fresh row when the key is new, which is how an event touches one account among thousands without the table being rebuilt");

pub fn entries<K: Clone, V: Clone>(map: &BTreeMap<K, V>) -> Vec<(K, V)> {
    map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}
because!(entries, "a keyed table as a list of pairs in key order, the form a store writes and a test compares");

pub fn rows<K, V: Clone>(map: &BTreeMap<K, V>) -> Vec<V> {
    map.values().cloned().collect()
}
because!(rows, "the values of a keyed table in key order without their keys, for a caller that wants the rows and already knows how they were keyed");

pub fn latest<K: Ord + Copy, V: Clone>(map: &BTreeMap<K, V>) -> Option<(K, V)> {
    map.iter().next_back().map(|(k, v)| (*k, v.clone()))
}
because!(latest, "the highest keyed entry of a table, which for a table keyed by day is the newest day it holds");

pub fn earliest<K: Ord + Copy, V: Clone>(map: &BTreeMap<K, V>) -> Option<(K, V)> {
    map.iter().next().map(|(k, v)| (*k, v.clone()))
}
because!(earliest, "the lowest keyed entry of a table, the oldest day of a series, which a lookup before the series began falls back to");

pub fn at_or_before<K: Ord + Copy, V: Clone>(map: &BTreeMap<K, V>, key: K) -> Option<V> {
    map.range(..=key).next_back().map(|(_, v)| v.clone())
}
because!(at_or_before, "the newest entry at or before a key, which is how a value known only on some days is carried forward onto the days between, in one ordered lookup rather than a scan");

pub fn from_key<K: Ord + Copy, V: Clone>(map: &BTreeMap<K, V>, key: K) -> Vec<(K, V)> {
    map.range(key..).map(|(k, v)| (*k, v.clone())).collect()
}
because!(from_key, "every entry of a table from a key onward in order, the tail of a series a reader asked for from a day");

pub fn removed<K: Ord, V>(map: BTreeMap<K, V>, key: &K) -> BTreeMap<K, V> {
    let mut out = map;
    out.remove(key);
    out
}
because!(removed, "a keyed table with one entry taken out, by value like placed_in, for the row an event retires");

pub fn merged<K: Ord, V>(into: BTreeMap<K, V>, from: BTreeMap<K, V>) -> BTreeMap<K, V> {
    let mut out = into;
    out.extend(from);
    out
}
because!(merged, "two keyed tables as one, the second winning on a shared key, which is how rows fetched for a batch are laid over the rows already held");

pub fn keys<K: Clone, V>(map: &BTreeMap<K, V>) -> Vec<K> {
    map.keys().cloned().collect()
}
because!(keys, "the keys of a table in order, for a caller that needs to know which days or which accounts a batch touched");

pub fn tabled<K: Ord, V>(pairs: Vec<(K, V)>) -> BTreeMap<K, V> {
    pairs.into_iter().collect()
}
because!(tabled, "a list of pairs as a keyed table, the reverse of entries, so a store's rows become the state a reducer works on");

pub fn count_in<K, V>(map: &BTreeMap<K, V>) -> usize {
    map.len()
}
because!(count_in, "how many entries a keyed table holds, the size of a table as size is the size of a list");

pub fn latest_value<K: Ord, V: Clone>(map: &BTreeMap<K, V>) -> Option<V> {
    map.values().next_back().cloned()
}
because!(latest_value, "the value under the highest key, or nothing when the table is empty, the newest reading a tile shows or leaves blank");

pub fn latest_value_or<K: Ord, V: Clone>(map: &BTreeMap<K, V>, fallback: V) -> V {
    map.values().next_back().cloned().unwrap_or(fallback)
}
because!(latest_value_or, "the value under the highest key or a fallback when the table is empty, today's price or zero");

pub fn union_keys<K: Ord + Copy, V>(maps: &[&BTreeMap<K, V>]) -> Vec<K> {
    let mut keys = std::collections::BTreeSet::new();
    for map in maps {
        keys.extend(map.keys().copied());
    }
    keys.into_iter().collect()
}
because!(union_keys, "every key any of several tables holds, once each and in order, the axis a chart over more than one series walks");

pub fn latest_key_or<K: Ord + Copy, V>(map: &BTreeMap<K, V>, fallback: K) -> K {
    map.keys().next_back().copied().unwrap_or(fallback)
}
because!(latest_key_or, "the highest key of a table or a fallback when it is empty, which is where a fetch of a day series resumes from: the newest day held, or today when nothing is held yet");

pub fn empty_table<K: Ord, V>() -> BTreeMap<K, V> {
    BTreeMap::new()
}
because!(empty_table, "a keyed table with nothing in it, the counterpart of an empty list, since a reducer's initial state may only call the library and a constructor is not a pattern");

pub fn empty_set<K: Ord>() -> std::collections::BTreeSet<K> {
    std::collections::BTreeSet::new()
}
because!(empty_set, "a set with nothing in it, the counterpart of empty_table for a reducer that only needs to know whether a pair has been seen");
