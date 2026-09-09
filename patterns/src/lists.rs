use patterns_macros::because;

pub fn paged<T: Clone>(fetched: &[T], per_page: usize) -> (Vec<T>, bool) {
    (fetched.iter().take(per_page).cloned().collect(), fetched.len() > per_page)
}
because!(paged, "one page out of one more item than the page holds, so the page knows another follows without a second count");

pub fn without_empty<K, T>(series: Vec<(K, Vec<T>)>) -> Vec<(K, Vec<T>)> {
    series.into_iter().filter(|(_, items)| !items.is_empty()).collect()
}
because!(without_empty, "the named lists that hold at least one item, since a chart with an empty line draws a legend entry for nothing");

pub fn holds_in<T: PartialEq>(items: &[T], value: T) -> bool {
    items.contains(&value)
}
because!(holds_in, "whether a list holds a value equal to the one given, for a list of plain values that carry no key of their own, where holds asks a keyed element for its key");

pub fn extended<T>(items: Vec<T>, more: Vec<T>) -> Vec<T> {
    let mut out = items;
    out.extend(more);
    out
}
because!(extended, "one list followed by another, taken by value like pushed, for the rows one event adds to the rows a batch has gathered");

pub fn joined_lists<T>(lists: Vec<Vec<T>>) -> Vec<T> {
    lists.into_iter().flatten().collect()
}
because!(joined_lists, "several lists as one in order, so a reducer that builds a movement per condition gathers them without a loop");

pub fn any_true(flags: &[bool]) -> bool {
    flags.iter().any(|f| *f)
}
because!(any_true, "whether any of several conditions holds, the or a binding may not write as an operator, paired with all_true which asks the same of every one");

pub fn all_true(flags: &[bool]) -> bool {
    flags.iter().all(|f| *f)
}
because!(all_true, "whether every one of several conditions holds, the and a binding may not write as an operator, paired with any_true which asks the same of any one");

pub fn is_some<T>(value: &Option<T>) -> bool {
    value.is_some()
}
because!(is_some, "whether a maybe holds a value, asked as a call because a method is not a pattern");

pub fn or_else<T: Copy>(value: Option<T>, fallback: T) -> T {
    value.unwrap_or(fallback)
}
because!(or_else, "a maybe's value or a fallback, so a reducer reads a lookup that may miss without a match of its own");

pub fn only_if<T>(when: bool, value: T) -> Option<T> {
    match when {
        true => Some(value),
        false => None,
    }
}
because!(only_if, "a value present only when a condition holds, the way a reducer states an optional touch or an optional row without a branch");

pub fn dedup_sorted<T: Ord + Clone>(items: &[T]) -> Vec<T> {
    let mut out = items.to_vec();
    out.sort();
    out.dedup();
    out
}
because!(dedup_sorted, "a list with each value once, in order, for the set of accounts a batch touches which a store is asked for in one request");

pub fn mapped_maybe<T, U>(value: Option<T>, each: fn(&T) -> U) -> Option<U> {
    value.as_ref().map(each)
}
because!(mapped_maybe, "a maybe's value passed through a function when present, so a binding that reads an optional fact and needs it in another form writes no match");

pub fn converted<A, B: From<A>>(items: Vec<A>) -> Vec<B> {
    items.into_iter().map(B::from).collect()
}
because!(converted, "a list of one shape as a list of another through the conversion a spec declares, so a binding turns what a pattern computed into what a page answers without a loop of its own");

pub fn made<A, B: From<A>>(item: A) -> B {
    B::from(item)
}
because!(made, "one value of one shape as another through the conversion a spec declares, the single form of converted");

pub fn zipped<A, B>(first: Vec<A>, second: Vec<B>) -> Vec<(A, B)> {
    first.into_iter().zip(second).collect()
}
because!(zipped, "two lists of equal length as one list of pairs, so a binding joins what two patterns computed for the same items");

pub fn zipped3<A, B, C>(first: Vec<A>, second: Vec<B>, third: Vec<C>) -> Vec<(A, B, C)> {
    first
        .into_iter()
        .zip(second)
        .zip(third)
        .map(|((a, b), c)| (a, b, c))
        .collect()
}
because!(zipped3, "three lists of equal length as one list of triples, zipped for the items that carry three computed parts");

pub fn zipped4<A, B, C, D>(first: Vec<A>, second: Vec<B>, third: Vec<C>, fourth: Vec<D>) -> Vec<(A, B, C, D)> {
    first
        .into_iter()
        .zip(second)
        .zip(third)
        .zip(fourth)
        .map(|(((a, b), c), d)| (a, b, c, d))
        .collect()
}
because!(zipped4, "four lists of equal length as one list of quadruples, for the items that carry four computed parts");

pub fn each_of<A, B>(items: &[A], read: fn(&A) -> B) -> Vec<B> {
    items.iter().map(read).collect()
}
because!(each_of, "one projection of every item, the same as map, named apart because a binding that reads a spec table for every item of a list reads better as each_of than as map");

pub fn each_named<A, B: Copy>(items: &[A], read: fn(&A) -> B) -> Vec<B> {
    items.iter().map(read).collect()
}
because!(each_named, "one copyable projection of every item, the copyable twin of each_of, kept apart so a table of values and a table of texts do not share one entry");

pub fn keep_where<A: Clone, B: Copy + PartialEq>(items: &[A], read: fn(&A) -> B, wanted: B) -> Vec<A> {
    items.iter().filter(|item| read(item) == wanted).cloned().collect()
}
because!(keep_where, "the items whose projection equals a value, for a binding that keeps the rows of one kind without a closure");

pub fn drop_where<A: Clone, B: Copy + PartialEq>(items: &[A], read: fn(&A) -> B, unwanted: B) -> Vec<A> {
    items.iter().filter(|item| read(item) != unwanted).cloned().collect()
}
because!(drop_where, "the items whose projection differs from a value, the other half of keep_where, for a menu that hides the entries of one kind");

pub fn paired<A: Clone, B: Clone>(items: &[A], with: &[B]) -> Vec<(A, B)> {
    items.iter().cloned().zip(with.iter().cloned()).collect()
}
because!(paired, "two borrowed lists as one list of cloned pairs, the borrowing twin of zipped for lists a binding does not own");

pub fn maybe_from<A, B: From<A>>(item: Option<A>) -> Option<B> {
    item.map(B::from)
}
because!(maybe_from, "an optional value converted when present, the optional twin of made");

pub fn is_blank(text: &str) -> bool {
    text.is_empty()
}
because!(is_blank, "whether a text has nothing in it, asked as a call, the test that turns an empty sublabel into no sublabel");

pub fn or_default<T: Default>(value: Option<T>) -> T {
    value.unwrap_or_default()
}
because!(or_default, "a maybe's value or the empty value of its type, for the owned texts and lists a binding may not copy through or_else");

pub fn each_with<A, C, B>(items: &[A], with: &C, read: fn(&A, &C) -> B) -> Vec<B> {
    items.iter().map(|item| read(item, with)).collect()
}
because!(each_with, "one projection of every item read beside a shared value, for a table whose every row needs the deployment's token names, since a binding may not close over them");

pub fn indexed_with<A, C, B>(items: &[A], with: &C, read: fn(usize, &A, &C) -> B) -> Vec<B> {
    items.iter().enumerate().map(|(at, item)| read(at, item, with)).collect()
}
because!(indexed_with, "each_with told the item's place as well, for a chart whose lines take their ink from their order");

pub fn after_prefix(text: &str, prefix: &str) -> Option<String> {
    text.strip_prefix(prefix).map(String::from)
}
because!(after_prefix, "what follows a prefix in a text, or nothing when the text does not begin with it, which is how a path yields the address it carries");

pub fn starts_with_text(text: &str, prefix: &str) -> bool {
    text.starts_with(prefix)
}
because!(starts_with_text, "whether a text begins with another, asked as a call, the test a route match makes on a path");

pub fn step_pick<T: Copy>(value: f64, steps: &[(f64, T)], low: T) -> T {
    steps.iter().find(|(at, _)| value >= *at).map(|(_, pick)| *pick).unwrap_or(low)
}
because!(step_pick, "the first entry whose threshold the value reaches, or the lowest entry, the glyph a percentage earns");

pub fn accumulated<T: Clone, K: PartialEq>(pages: &[Vec<T>], key: fn(&T) -> K) -> Vec<T> {
    let mut out: Vec<T> = Vec::new();
    for page in pages {
        for item in page {
            let seen = out.iter().any(|kept| key(kept) == key(item));
            if !seen {
                out.push(item.clone());
            }
        }
    }
    out
}
because!(accumulated, "every page's rows in order with a row seen on an earlier page left out, the table a reader has loaded more of");

pub fn flattened<T: Clone>(pages: &[Vec<T>]) -> Vec<T> {
    pages.iter().flat_map(|page| page.iter().cloned()).collect()
}
because!(flattened, "every page's rows in order, the plain twin of accumulated for rows that carry no key");

pub fn last_of<T: Clone>(items: &[T]) -> Option<T> {
    items.last().cloned()
}
because!(last_of, "the last item of a list, or nothing, the newest page a reader has loaded");

pub fn count_of_list<T>(items: &[T]) -> u64 {
    u64::try_from(items.len()).unwrap_or(u64::MAX)
}
because!(count_of_list, "how many items a list holds, as the number a page shows, the sizing a binding may not write as a cast");

pub fn either_text(when: bool, yes: &str, no: &str) -> String {
    match when {
        true => String::from(yes),
        false => String::from(no),
    }
}
because!(either_text, "one of two texts by a flag, owned, so a binding chooses a word without a match");

pub fn joined_texts(parts: &[String]) -> String {
    parts.concat()
}
because!(joined_texts, "several texts run together with nothing between, the way a label with a figure in the middle is assembled");

pub fn text_if(when: bool, text: &str) -> String {
    match when {
        true => String::from(text),
        false => String::new(),
    }
}
because!(text_if, "a text when a flag holds and nothing otherwise, the optional tail of a label");

pub fn found_in<K: PartialEq, V: Copy>(table: &[(K, V)], key: &K, fallback: V) -> V {
    table.iter().find(|(k, _)| k == key).map(|(_, v)| *v).unwrap_or(fallback)
}
because!(found_in, "the value paired with a key in a small table, or a fallback, the ink a series name earns");

pub fn swapped_in<K: PartialEq + Clone, V: Clone>(table: &[(K, V)], key: &K, value: V) -> Vec<(K, V)> {
    table
        .iter()
        .map(|(k, v)| match k == key {
            true => (k.clone(), value.clone()),
            false => (k.clone(), v.clone()),
        })
        .collect()
}
because!(swapped_in, "a small table with one key's value swapped and every other pair left alone, which is how one option of a control carries the whole reading with only its own value moved");

pub fn interleaved<T: Clone>(items: &[T], between: &T) -> Vec<T> {
    let mut out = Vec::new();
    for (at, item) in items.iter().enumerate() {
        if at > 0 {
            out.push(between.clone());
        }
        out.push(item.clone());
    }
    out
}
because!(interleaved, "a list with a separator put between every two items, the runs of a chart title with the slashes between the token names");

pub fn zipped_with<A: Clone, B: Clone, C, D>(firsts: &[A], seconds: &[B], with: &C, read: fn(&A, &B, &C) -> D) -> Vec<D> {
    firsts.iter().zip(seconds.iter()).map(|(a, b)| read(a, b, with)).collect()
}
because!(zipped_with, "one reading of every pair of two lists beside a shared value, a chart definition beside the payload that fills it");

pub fn any_some<T>(items: &[Option<T>]) -> bool {
    items.iter().any(|item| item.is_some())
}
because!(any_some, "whether any item of a list is present, the test that says a page's payload has arrived");

pub fn taken_from<T: Clone>(items: &[T], count: usize) -> Vec<T> {
    items.iter().take(count).cloned().collect()
}
because!(taken_from, "the first count items of a list, the tokens a title names when the payload holds fewer lines than the title could");

pub fn mapped_with<T, C, U>(value: &Option<T>, with: &C, read: fn(&T, &C) -> U) -> Option<U> {
    value.as_ref().map(|inner| read(inner, with))
}
because!(mapped_with, "a maybe's value passed through a function beside a shared value, so a binding reads an optional payload with the deployment's names in hand and closes over nothing");

pub fn first_where<T: Clone>(items: &[T], keep: fn(&T) -> bool) -> Option<T> {
    items.iter().find(|item| keep(item)).cloned()
}
because!(first_where, "the first item a test keeps, or nothing, the chosen entry of a menu");

pub fn flattened_maybe<T>(value: Option<Option<T>>) -> Option<T> {
    value.flatten()
}
because!(flattened_maybe, "a maybe inside a maybe as one maybe, the shape an optional payload's optional field reads as");

pub fn highest_value(values: &[f64]) -> f64 {
    values.iter().copied().fold(crate::money::none(), f64::max)
}
because!(highest_value, "the largest of some figures, or zero for none, the tallest bar an axis is sized by");

pub fn places_of(value: u64) -> usize {
    usize::try_from(value).ok().unwrap_or_default()
}
because!(places_of, "a place count read from a payload as the width a formatter takes");

pub fn is_non_empty<T>(items: &[T]) -> bool {
    !items.is_empty()
}
because!(is_non_empty, "whether a list holds anything, the test that says a table has loaded a page");

pub fn found_maybe<K: PartialEq, V: Copy>(table: &[(K, V)], key: &K) -> Option<V> {
    table.iter().find(|(k, _)| k == key).map(|(_, v)| *v)
}
because!(found_maybe, "the value paired with a key in a small table, or nothing, the cell a spreadsheet leaves empty");

pub fn picked<K: PartialEq, V: Clone>(table: &[(K, V)], key: &K, fallback: V) -> V {
    for (k, v) in table {
        if k == key {
            return v.clone();
        }
    }
    fallback
}
because!(picked, "the value paired with a key in a small table, or a fallback, for the owned values found_in cannot copy, a heading or a row's runs chosen by the sort");

pub fn or_owned<T>(value: Option<T>, fallback: T) -> T {
    match value {
        Some(inner) => inner,
        None => fallback,
    }
}
because!(or_owned, "a maybe's value or a fallback, for the owned values or_else cannot copy, a menu entry or a glyph");

pub fn cloned<T: Clone>(value: &T) -> T {
    T::clone(value)
}
because!(cloned, "an owned copy of a borrowed value, the copy a binding may not make with a method");

pub fn ranked_by<T>(items: Vec<T>, key: fn(&T) -> f64, descending: bool) -> Vec<T> {
    let mut out = items;
    out.sort_by(|a, b| {
        let (x, y) = (key(a), key(b));
        let cmp = x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal);
        match descending {
            true => cmp.reverse(),
            false => cmp,
        }
    });
    out
}
because!(ranked_by, "the items in the order of one figure, either way, with a stable sort so ties keep the order the items arrived in");

pub fn kept<T>(items: Vec<T>, keep: fn(&T) -> bool) -> Vec<T> {
    items.into_iter().filter(keep).collect()
}
because!(kept, "the items a rule keeps, by a plain function so a binding names the rule from a spec table");

pub fn page_with_count<T: Clone>(items: &[T], page: usize, per_page: usize) -> (Vec<T>, bool, usize) {
    let start = page.saturating_mul(per_page).min(items.len());
    let end = start.saturating_add(per_page).min(items.len());
    (items[start..end].to_vec(), end < items.len(), items.len())
}
because!(page_with_count, "one page of an ordered list with whether another follows and how many items there are in all, sliced from the whole because the list is small enough to hold");

pub fn bars<T>(items: &[T], keep: fn(&T) -> bool, label: fn(&T, usize) -> String, value: fn(&T) -> f64, count: usize, label_chars: usize, places: u32) -> Vec<(String, f64)> {
    items
        .iter()
        .filter(|item| keep(item))
        .take(count)
        .map(|item| (label(item, label_chars), crate::money::rounded(value(item), places)))
        .collect()
}
because!(bars, "the first few kept items of an order as labelled bars, the chart above a table");

pub fn bars_maybe<T>(items: &[T], keep: fn(&T) -> bool, label: fn(&T, usize) -> String, value: Option<fn(&T) -> f64>, count: usize, label_chars: usize, places: u32) -> Option<Vec<(String, f64)>> {
    value.map(|read| bars(items, keep, label, read, count, label_chars, places))
}
because!(bars_maybe, "the bar chart of an order when the order has one, and nothing when it has none, so a table answers either without a branch");
