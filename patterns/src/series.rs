use crate::seq::Facet;
use patterns_macros::because;
use std::collections::BTreeMap;

#[derive(patterns_macros::Record)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Running {
    pub amount: i128,
    pub count: i64,
}
because!(Running, "a total that events move, signed because an unlock takes back what a lock added and a count of open positions can go under zero when its opening was never seen");

#[derive(patterns_macros::Record)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DayTotal {
    pub amount: i128,
    pub count: i64,
    pub ending_amount: i128,
    pub ending_count: i64,
}
because!(DayTotal, "what one day added to a running total and where the total stood at the day's end, the two readings a chart needs: the bar and the line");

pub type Days<S> = BTreeMap<(S, u64), DayTotal>;

pub type Totals<S> = BTreeMap<S, Running>;

pub fn booked<S: Ord + Copy>(days: Days<S>, totals: Totals<S>, series: S, day: u64, amount: i128, count: i64) -> (Days<S>, Totals<S>) {
    let mut totals = totals;
    let mut days = days;
    let running = totals.entry(series).or_default();
    running.amount = running.amount.saturating_add(amount);
    running.count = running.count.saturating_add(count);
    let ending = *running;
    let row = days.entry((series, day)).or_default();
    row.amount = row.amount.saturating_add(amount);
    row.count = row.count.saturating_add(count);
    row.ending_amount = ending.amount;
    row.ending_count = ending.count;
    (days, totals)
}
because!(booked, "one movement written to a series: the running total moves, and the day's row takes the movement and the total the day now ends at, so the bar and the line can never disagree");

pub fn booked_all<S: Ord + Copy>(days: Days<S>, totals: Totals<S>, day: u64, bookings: &[(S, i128, i64)]) -> (Days<S>, Totals<S>) {
    let mut out = (days, totals);
    for (series, amount, count) in bookings {
        out = booked(out.0, out.1, *series, day, *amount, *count);
    }
    out
}
because!(booked_all, "several movements of one event written in order, so an event that moves two series is one call in a reducer");

pub fn moved_total<S: Ord + Copy>(totals: Totals<S>, series: S, amount: i128) -> Totals<S> {
    let mut out = totals;
    let running = out.entry(series).or_default();
    running.amount = running.amount.saturating_add(amount);
    out
}
because!(moved_total, "a running total moved with no day row written, for supply that arrives from outside the program's own events and belongs to the total but to no day of the chart");

pub fn amount_of<S: Ord>(totals: &Totals<S>, series: &S) -> i128 {
    totals.get(series).map(|r| r.amount).unwrap_or(0)
}
because!(amount_of, "where a running total stands, or zero for a series nothing has moved yet, the other half of count_of");

pub fn count_of<S: Ord>(totals: &Totals<S>, series: &S) -> i64 {
    totals.get(series).map(|r| r.count).unwrap_or(0)
}
because!(count_of, "how many a running total counts, the other half of amount_of, zero for a series nothing has moved yet");

pub fn series_days<S: Ord + Copy>(days: &Days<S>, series: S) -> Vec<(u64, DayTotal)> {
    days.iter()
        .filter(|((s, _), _)| *s == series)
        .map(|((_, day), row)| (*day, *row))
        .collect()
}
because!(series_days, "the day rows of one series in day order, out of a table that keys every series together");

pub fn tallied<F, T: Facet<F>>(totals: BTreeMap<T::Value, u64>, rows: &[T]) -> BTreeMap<T::Value, u64>
where
    T::Value: Ord + Copy,
{
    let mut out = totals;
    for row in rows {
        *out.entry(row.facet()).or_default() += 1;
    }
    out
}
because!(tallied, "a count per kind raised by one for each row of that kind, the totals a feed shows in its filter menu without counting the feed");

pub fn last_facet_or<F, T: Facet<F>>(items: &[T], fallback: T::Value) -> T::Value {
    items.last().map(|item| item.facet()).unwrap_or(fallback)
}
because!(last_facet_or, "the projection of a list's last element, or a fallback for an empty list, which is where a walk's cursor moves to once its rows are priced");
