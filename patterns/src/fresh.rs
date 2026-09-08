use crate::seq::Shift;
use patterns_macros::because;
use std::collections::BTreeMap;

pub trait Fresh<K> {
    fn fresh(key: &K) -> Self;
}
because!(Fresh, "how a row is made for a key a table has never seen, declared by the spec that owns the row, so a table can move a row that does not exist yet");

pub fn touched<M, K: Ord + Clone, V: Shift<M> + Fresh<K>>(map: BTreeMap<K, V>, key: K, by: V::By) -> BTreeMap<K, V> {
    let mut out = map;
    let current = out.remove(&key).unwrap_or_else(|| V::fresh(&key));
    out.insert(key, current.shifted(by));
    out
}
because!(touched, "one row of a keyed table moved by a step, made fresh first when the key is new, which is how an event reaches one account among thousands without the table being rebuilt");

pub fn touched_maybe<M, K: Ord + Clone, V: Shift<M> + Fresh<K>>(map: BTreeMap<K, V>, by: &Option<(K, V::By)>) -> BTreeMap<K, V> {
    match by {
        Some((key, step)) => touched(map, key.clone(), *step),
        None => map,
    }
}
because!(touched_maybe, "touched when an event names a row to move and the table unchanged when it does not, so a reducer with an optional touch needs no match of its own");

pub fn touched_all<M, K: Ord + Clone, V: Shift<M> + Fresh<K>>(map: BTreeMap<K, V>, steps: &[(K, V::By)]) -> BTreeMap<K, V> {
    let mut out = map;
    for (key, step) in steps {
        out = touched(out, key.clone(), *step);
    }
    out
}
because!(touched_all, "several rows moved in order by one event, for an event that reaches two accounts at once, the sender and the receiver");

pub fn looked_or<K: Ord, V: Clone>(map: &BTreeMap<K, V>, key: &K, fallback: V) -> V {
    map.get(key).cloned().unwrap_or(fallback)
}
because!(looked_or, "one entry of a keyed table or a fallback when the key is absent, for a reducer that reads a row that may not exist yet");

pub fn looked_fresh<K: Ord, V: Clone + Fresh<K>>(map: &BTreeMap<K, V>, key: &K) -> V {
    map.get(key).cloned().unwrap_or_else(|| V::fresh(key))
}
because!(looked_fresh, "one row of a keyed table, or the fresh row its key would start from, so a reducer reads an account's counters before its first event as zeros without a match");
