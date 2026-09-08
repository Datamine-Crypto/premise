use crate::arith::{raise_by, reduce_by, rem, Floor};
use crate::select::{is_at_least, is_at_most};
use patterns_macros::because;

pub fn is_within<T: PartialOrd>(v: T, lo: T, hi: T) -> bool {
    v >= lo && v < hi
}
because!(is_within, "membership in a window as one call, half open at the top so adjoining windows are disjoint, and separate from two comparisons because the closed end is the common error");

pub fn fits<T: Floor + PartialOrd>(used: T, more: T, cap: T) -> bool {
    is_at_most(raise_by(used, more), cap)
}
because!(fits, "a spend held against its budget before it is made, the scalar form of is_full, so an allowance check is one call and the saturating add cannot wrap into a false yes");

pub fn is_due<T: Floor + PartialOrd>(last: T, every: T, now: T) -> bool {
    is_at_least(now, raise_by(last, every))
}
because!(is_due, "whether an interval has elapsed since a last occurrence, so a recurring charge, reminder or inspection asks one question and the boundary day counts as due");

pub fn cycled<T: Floor + Copy>(v: T, lo: T, hi: T) -> Option<T> {
    rem(reduce_by(v, lo), reduce_by(hi, lo)).map(|turn| raise_by(lo, turn))
}
because!(cycled, "a value folded back into a range instead of pinned at its edge, which clamp_range is not, because a weekday or a bay in a ring comes round again; a value under the floor lands on the floor since the distance saturates");

pub fn folded<T: Floor + Copy>(value: T, floor: T, step: T) -> Option<T> {
    cycled(value, floor, raise_by(floor, step))
}
because!(folded, "a value brought into one window by whole steps of a stated size, where cycled folds by the width of a range instead, so a note folded into an octave or a reading folded into a cycle names the step it moves by rather than the two ends of the window");
