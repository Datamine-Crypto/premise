use crate::arith::{raise_by, Floor};
use patterns_macros::because;

pub fn map<T, U>(items: &[T], each: fn(&T) -> U) -> Vec<U> {
    let mut out = Vec::new();
    for item in items {
        out.push(each(item));
    }
    out
}

pub fn keep<T: Clone>(items: &[T], when: fn(&T) -> bool) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        if when(item) {
            out.push(item.clone());
        }
    }
    out
}

pub fn fold<T, A>(items: &[T], start: A, step: fn(A, &T) -> A) -> A {
    let mut acc = start;
    for item in items {
        acc = step(acc, item);
    }
    acc
}

pub fn count<T>(items: &[T], when: fn(&T) -> bool) -> usize {
    let mut total = 0usize;
    for item in items {
        if when(item) {
            total += 1;
        }
    }
    total
}

pub fn any<T>(items: &[T], when: fn(&T) -> bool) -> bool {
    for item in items {
        if when(item) {
            return true;
        }
    }
    false
}

pub fn all<T>(items: &[T], when: fn(&T) -> bool) -> bool {
    for item in items {
        if !when(item) {
            return false;
        }
    }
    true
}

pub fn first<T: Clone>(items: &[T], when: fn(&T) -> bool) -> Option<T> {
    for item in items {
        if when(item) {
            return Some(item.clone());
        }
    }
    None
}

pub trait Keyed {
    type Key: PartialEq;

    fn key(&self) -> Self::Key;
}

pub trait Advance {
    type By: Copy;

    fn advanced(self, by: Self::By) -> Self;
}

pub trait Shift<M> {
    type By: Copy;

    fn shifted(self, by: Self::By) -> Self;
}


pub fn is_empty<T>(items: &[T]) -> bool {
    items.is_empty()
}

pub fn size<T>(items: &[T]) -> usize {
    items.len()
}

pub fn is_full<T>(items: &[T], capacity: usize) -> bool {
    items.len() >= capacity
}

pub fn holds<T: Keyed>(items: &[T], key: T::Key) -> bool {
    for item in items {
        if item.key() == key {
            return true;
        }
    }
    false
}

pub fn found<T: Keyed + Clone>(items: &[T], key: T::Key) -> Option<T> {
    for item in items {
        if item.key() == key {
            return Some(item.clone());
        }
    }
    None
}

pub fn with<T: Clone>(items: &[T], item: T) -> Vec<T> {
    let mut out = items.to_vec();
    out.push(item);
    out
}

pub fn without<T: Keyed + Clone>(items: &[T], key: T::Key) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        if item.key() != key {
            out.push(item.clone());
        }
    }
    out
}

pub fn advance_all<T: Advance + Clone>(items: &[T], by: T::By) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        out.push(item.clone().advanced(by));
    }
    out
}

pub fn shift_all<M, T: Shift<M> + Clone>(items: &[T], by: T::By) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        out.push(item.clone().shifted(by));
    }
    out
}

pub trait Ranked {
    type Rank: PartialOrd;

    fn rank(&self) -> Self::Rank;
}

pub fn count_above<T: Ranked>(items: &[T], bound: T::Rank) -> usize {
    let mut seen = 0usize;
    for item in items {
        if item.rank() > bound {
            seen += 1;
        }
    }
    seen
}

pub fn keep_above<T: Ranked + Clone>(items: &[T], bound: T::Rank) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        if item.rank() > bound {
            out.push(item.clone());
        }
    }
    out
}

pub fn rank_of<T: Ranked>(value: &T) -> T::Rank {
    value.rank()
}

pub trait Facet<F> {
    type Value: PartialEq;

    fn facet(&self) -> Self::Value;
}

pub fn count_facet<F, T: Facet<F>>(items: &[T], value: T::Value) -> usize {
    let mut seen = 0usize;
    for item in items {
        if item.facet() == value {
            seen += 1;
        }
    }
    seen
}

pub fn any_facet<F, T: Facet<F>>(items: &[T], value: T::Value) -> bool {
    for item in items {
        if item.facet() == value {
            return true;
        }
    }
    false
}

pub fn first_facet<F, T: Facet<F> + Clone>(items: &[T], value: T::Value) -> Option<T> {
    for item in items {
        if item.facet() == value {
            return Some(item.clone());
        }
    }
    None
}
because!(map, "the replacement for a for loop that builds a list, taking a fn pointer rather than a closure so nothing can capture");
because!(keep, "filtering by a predicate, separate from map because a filter changes the length and a map does not");
because!(fold, "the general accumulation, kept because map and keep cannot express a running total");
because!(count, "folding to a count, named separately because counting is common enough that spelling it as a fold obscures the intent");
because!(any, "existence over a predicate, which short circuits and so is not count compared against zero");
because!(all, "universality over a predicate, the mirror of any and separate because the empty case answers differently");
because!(first, "the first element satisfying a predicate, returning an Option so the empty case is a value rather than a panic");
because!(is_empty, "emptiness as a call, so a length comparison never appears in spec or app");
because!(size, "length as a call, so a length comparison never appears in spec or app");
because!(is_full, "a collection held against its capacity, so a capacity is a usize in exactly one place and a fullness test is a call rather than a comparison");
because!(holds, "membership by key, so a caller asks whether the collection already has this thing without writing the comparison");
because!(found, "retrieval by key, separate from holds because the caller that wants the item should not have to search twice");
because!(with, "adding an element and returning the new collection, so a binding never mutates");
because!(without, "removing by key and returning the new collection, the mirror of with");
because!(advance_all, "advancing every element by its own rule, so a caller never loops to apply a step to a collection");
because!(count_above, "counting elements ranked over a bound, which is count composed with a comparison the caller would otherwise write as a closure");
because!(keep_above, "the filtering form of count_above, kept separate because a caller usually wants either the number or the survivors and not both");
because!(rank_of, "the rank a single element projects to, exposed so a caller can order by the same projection the collection functions use");
because!(count_facet, "counting elements whose facet matches, the equality form of count_above and the route for a predicate over a named field");
because!(any_facet, "existence over a facet, short circuiting, so a caller asks whether any element matches without counting them all");
because!(first_facet, "the first element whose facet matches, returning an Option so the empty case is a value rather than a panic");
because!(Shift, "the marker parameterised form of Advance, because coherence allows one Advance impl per type and a value that needs two different mutations cannot have them");
because!(shift_all, "advancing every element by one named mutation, so a type with several ways to move keeps them apart instead of merging them into one enum");

pub fn count_below<T: Ranked>(items: &[T], bound: T::Rank) -> usize {
    let mut seen = 0usize;
    for item in items {
        if item.rank() < bound {
            seen += 1;
        }
    }
    seen
}
because!(count_below, "counting elements ranked under a bound, the mirror of count_above and separate because a caller asking how many are short of a limit should not have to invert the question");

pub fn count_from<T: Ranked>(items: &[T], bound: T::Rank) -> usize {
    let mut seen = 0usize;
    for item in items {
        if item.rank() >= bound {
            seen += 1;
        }
    }
    seen
}
because!(count_from, "counting elements ranked at or over a bound, separate from count_above because the boundary case is exactly what a limit turns on and inverting a strict comparison is a common error");

pub trait Valued<M> {
    type Value: Copy;

    fn value(&self) -> Self::Value;
}
because!(Valued, "a lookup from a vocabulary variant to the figure it stands for, kept apart from Ranked because a cap, a rate or an allowance is read and never ordered, and the marker lets one variant carry several such figures");

pub fn value_of<M, T: Valued<M>>(item: &T) -> T::Value {
    item.value()
}
because!(value_of, "the figure a variant stands for, as a call, so the table a vocabulary carries is read from spec and app without a method call");

pub fn keep_facet<F, T: Facet<F> + Clone>(items: &[T], value: T::Value) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        if item.facet() == value {
            out.push(item.clone());
        }
    }
    out
}
because!(keep_facet, "the elements whose facet matches, the equality form of keep_above and the route for keeping every match on a named field without a closure");

pub fn without_facet<F, T: Facet<F> + Clone>(items: &[T], value: T::Value) -> Vec<T> {
    let mut out = Vec::new();
    for item in items {
        if item.facet() != value {
            out.push(item.clone());
        }
    }
    out
}
because!(without_facet, "removing every element whose facet matches, the mirror of keep_facet, and separate from without because a facet can be a tuple over several fields where a key is one identity");

pub fn last<T: Clone>(items: &[T], when: fn(&T) -> bool) -> Option<T> {
    let mut out = None;
    for item in items {
        if when(item) {
            out = Some(item.clone());
        }
    }
    out
}
because!(last, "the final element satisfying a predicate, separate from first because a log is read from its end as often as its start and reversing the list to ask copies it");

pub fn total<M, T: Valued<M>>(items: &[T], start: T::Value) -> T::Value
where
    T::Value: Floor,
{
    let mut sum = start;
    for item in items {
        sum = raise_by(sum, item.value());
    }
    sum
}
because!(total, "the sum of every value a marker reads from the elements, seeded by the caller and saturating, so a total of money or days stops at the type ceiling rather than wrapping and the pattern never conjures a zero; it reads Valued and not Ranked because a rank orders and a value is added");

pub fn found_ref<T: Keyed>(items: &[T], key: T::Key) -> Option<&T> {
    items.iter().find(|item| item.key() == key)
}
because!(found_ref, "retrieval by key that borrows the element, beside found because a query over a large state should not copy the element it reads while a reducer that keeps the element wants its own copy");

pub fn first_ref<T>(items: &[T], when: fn(&T) -> bool) -> Option<&T> {
    items.iter().find(|item| when(item))
}
because!(first_ref, "the first element satisfying a predicate, borrowed, beside first because a caller that only reads the element should not pay for a clone and a type that is not Clone still needs the search");

pub fn keep_ref<T>(items: &[T], when: fn(&T) -> bool) -> Vec<&T> {
    let mut out = Vec::new();
    for item in items {
        if when(item) {
            out.push(item);
        }
    }
    out
}
because!(keep_ref, "filtering that borrows the survivors, beside keep because a query that only counts or reads what passes should not copy the elements it passes over");

pub fn first_facet_ref<F, T: Facet<F>>(items: &[T], value: T::Value) -> Option<&T> {
    items.iter().find(|item| item.facet() == value)
}
because!(first_facet_ref, "the first element whose facet matches, borrowed, beside first_facet because a binding that reads one field of the match should not clone the whole element to get it");

pub fn at<T: Clone>(items: &[T], index: usize) -> Option<T> {
    items.get(index).cloned()
}
because!(at, "the item in a position if the position is there, so a binding reaches into a sequence without the square brackets it may not write and without the chance of ending the program on a position that is not");

pub fn firsts<A: Clone, B>(pairs: &[(A, B)]) -> Vec<A> {
    pairs.iter().map(|(a, _)| a.clone()).collect()
}
because!(firsts, "the left of every pair, the twin of seconds and separate from map because a pair has no facet to name, so what is wanted is said by which side is taken rather than by a function that reaches in");

pub fn seconds<A, B: Clone>(pairs: &[(A, B)]) -> Vec<B> {
    pairs.iter().map(|(_, b)| b.clone()).collect()
}
because!(seconds, "the right of every pair, the twin of firsts, and separate from map because a pair carries no facet a function could name; the two sides are kept apart because a side taken by its number reads as nothing at the call");

pub fn pushed<T>(items: Vec<T>, item: T) -> Vec<T> {
    let mut out = items;
    out.push(item);
    out
}
because!(pushed, "a list with one more element, taking the list by value so a reducer that owns its state appends without copying what it already holds, where with copies for a caller that only borrows");

pub fn without_trailing_facet<F, T: Facet<F> + Clone>(items: &[T]) -> Vec<T> {
    let last = match items.last() {
        Some(item) => item.facet(),
        None => return vec![],
    };
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        if item.facet() != last {
            out.push(item.clone());
        }
    }
    out
}
because!(without_trailing_facet, "a list with every element of its last group removed, the group being whatever the projection says, so a capped fetch keeps only groups it is sure it saw whole");

pub fn listed<T: Clone>(items: &[T]) -> Vec<T> {
    items.to_vec()
}
because!(listed, "a borrowed run of elements as an owned list, for a reducer that must hand a copy into an event it builds and may not call a method to do it");
