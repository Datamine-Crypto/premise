use patterns_macros::because;

pub fn signed_of(value: u128) -> i128 {
    i128::try_from(value).unwrap_or(i128::MAX)
}
because!(signed_of, "an unsigned amount as a signed one so it can move a total that may also be taken from, pinned at the ceiling rather than wrapped to a negative");

pub fn negated(value: u128) -> i128 {
    signed_of(value).saturating_neg()
}
because!(negated, "an unsigned amount as the signed movement that takes it away, the form an unlock or a withdrawal books");

pub fn unsigned_of(value: i128) -> u128 {
    u128::try_from(value).unwrap_or(0)
}
because!(unsigned_of, "a signed total read back as an amount, with anything under zero read as nothing, since a formula over amounts has no meaning for a total that went negative");

pub fn highest_of<T: PartialOrd>(a: T, b: T) -> T {
    match a > b {
        true => a,
        false => b,
    }
}
because!(highest_of, "the larger of two values, for a last-active moment that only ever moves forward");

pub fn lowest_of<T: PartialOrd>(a: T, b: T) -> T {
    match a < b {
        true => a,
        false => b,
    }
}
because!(lowest_of, "the smaller of two values, the other side of highest_of, for a window that must not pass a cap");

pub fn wide_summed(a: u128, b: u128) -> u128 {
    a.saturating_add(b)
}
because!(wide_summed, "two amounts as one, the add a binding cannot write as an operator, pinned at the ceiling like every add in this library");

pub fn wide_taken(a: u128, b: u128) -> u128 {
    a.saturating_sub(b)
}
because!(wide_taken, "one amount less another, stopping at nothing, the take a binding cannot write as an operator");

pub fn negative_count(value: u64) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX).saturating_neg()
}
because!(negative_count, "a count as the signed step that takes it off a tally, the one an unlock books against the open positions");

pub fn summed_signed(total: i128, delta: i128) -> i128 {
    total.checked_add(delta).unwrap_or(total)
}
because!(summed_signed, "a signed total moved by a signed step, left where it was when the step would pass the edge, for the two totals a badge judge keeps that events push both ways");

pub fn unsigned_count(value: i64) -> u64 {
    u64::try_from(value.max(0)).unwrap_or(0)
}
because!(unsigned_count, "a signed count as the unsigned one a page shows, with a count below zero shown as none, since a page never shows a negative number of validators");
