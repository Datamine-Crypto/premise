use crate::arith::{away, div, mul, raise_by, reduce_by, rem, Floor};
use crate::clamp::clamp_upper;
use crate::select::{either, is_above, is_same};
use crate::seq::{Facet, Valued};
use patterns_macros::because;

pub fn tally<F, T: Facet<F>>(items: &[T]) -> Vec<(T::Value, usize)>
where
    T::Value: Clone,
{
    let mut out: Vec<(T::Value, usize)> = Vec::new();
    for item in items {
        let value = item.facet();
        let mut seen = false;
        for row in out.iter_mut() {
            if row.0 == value {
                row.1 += 1;
                seen = true;
            }
        }
        if !seen {
            out.push((value, 1));
        }
    }
    out
}
because!(tally, "a count per facet value in one pass, so a report by category needs no list of categories beforehand and each value appears once, in the order first seen");

pub fn running_total<M, T: Valued<M>>(items: &[T], start: T::Value) -> Vec<T::Value>
where
    T::Value: Floor,
{
    let mut out = Vec::new();
    let mut sum = start;
    for item in items {
        sum = raise_by(sum, item.value());
        out.push(sum);
    }
    out
}
because!(running_total, "the balance after every step from a seed the caller gives, reading the same marker value that total sums so the last line of a statement equals its total, and the pattern never conjures a zero");

pub fn share_of<T: Floor + Copy>(amount: T, part: T, whole: T) -> Option<T> {
    match (div(amount, whole), rem(amount, whole)) {
        (Some(wholes), Some(left)) => {
            div(mul(left, part), whole).map(|scrap| raise_by(mul(wholes, part), scrap))
        }
        _ => None,
    }
}
because!(share_of, "a proportional share of an amount, split into a whole quotient and a remainder before scaling: the remainder term cannot overflow because a remainder is under the whole, while the quotient term saturates through mul; truncated because a rounding rule is a fact the spec states");

pub fn div_up<T: Floor + PartialEq + Copy>(value: T, by: T) -> Option<T> {
    match (div(value, by), rem(value, by)) {
        (Some(whole), Some(left)) => Some(either(
            is_same(left, T::ZERO),
            whole,
            raise_by(whole, T::ONE),
        )),
        _ => None,
    }
}
because!(div_up, "division rounded toward the ceiling, separate from div because a started period is charged in full and rounding down is the common error in a tariff");

pub fn backoff<T: Floor + PartialOrd>(delay: T, factor: T, cap: T) -> T {
    clamp_upper(mul(delay, factor), cap)
}
because!(backoff, "the next wait in a retry schedule, scaled then capped in one call so a schedule cannot grow past its ceiling and the two steps are never written in the wrong order");

pub fn between<T: Floor + Copy + PartialOrd>(from: T, to: T, part: T, whole: T) -> Option<T> {
    let moved = share_of(away(from, to), part, whole)?;
    Some(either(is_above(to, from), raise_by(from, moved), reduce_by(from, moved)))
}
because!(between, "a point some part of the way from one value to another, which a colour that drifts, a meter that fills and an arc that counts down are all the same question wearing different clothes; the direction is read from the two ends rather than asked for, so a run backwards needs no second function");
