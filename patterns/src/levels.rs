use crate::money::{none, whole_tokens};
use crate::series::DayTotal;
use crate::table::at_or_before;
use patterns_macros::because;
use std::collections::BTreeMap;

pub type Prices = BTreeMap<u64, f64>;

pub type Rows = BTreeMap<u64, DayTotal>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Units {
    pub wei_per_unit: u128,
    pub units_per_token: f64,
}
because!(Units, "how an amount in smallest units becomes a number of tokens, carried as one value so every chart truncates to the same places");

pub fn whole(amount: u128, units: Units) -> f64 {
    whole_tokens(amount, units.wei_per_unit, units.units_per_token)
}
because!(whole, "an amount as whole tokens under the units a spec states, the one call every chart makes before it prices anything");

pub fn signed_whole(amount: i128, units: Units) -> f64 {
    match amount < 0 {
        true => -whole(amount.unsigned_abs(), units),
        false => whole(amount.unsigned_abs(), units),
    }
}
because!(signed_whole, "a signed total as tokens, negative when the total is, since a lock series can run under zero and a chart draws where it went");

pub fn ending_on(rows: &Rows, day: u64, units: Units) -> f64 {
    rows.range(..=day)
        .next_back()
        .map(|(_, row)| signed_whole(row.ending_amount, units))
        .unwrap_or(none())
}
because!(ending_on, "where a series stood at the end of a day, carried forward from the newest row at or before it, and nothing before the series began");

pub fn row_on(rows: &Rows, day: u64) -> Option<DayTotal> {
    at_or_before(rows, day)
}
because!(row_on, "the newest row of a series at or before a day, for a reader that needs the counts as well as the amount");

pub fn latest_ending(rows: &Rows, units: Units) -> f64 {
    rows.iter()
        .next_back()
        .map(|(_, row)| signed_whole(row.ending_amount, units))
        .unwrap_or(none())
}
because!(latest_ending, "where a series stands today, its last row's ending amount as tokens, or nothing for an empty series");

pub fn day_amount(rows: &Rows, day: u64, units: Units) -> f64 {
    rows.get(&day)
        .map(|row| signed_whole(row.amount, units))
        .unwrap_or(none())
}
because!(day_amount, "what a series moved on one exact day as tokens, zero on a day it has no row, for the readers that sum movements rather than read levels");

pub fn latest_tally(rows: &Rows, ending: bool) -> Option<u64> {
    rows.iter().next_back().map(|(_, row)| {
        let count = match ending {
            true => row.ending_count,
            false => row.count,
        };
        count.max(0) as u64
    })
}
because!(latest_tally, "the newest row's count or ending count as a whole number, the figure the metric menu shows beside an entry, or nothing for an empty series");

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeiRounding {
    pub scale: f64,
    pub wei_per_scaled: u128,
}
because!(WeiRounding, "how a number of tokens is turned back into smallest units for the formula, rounded at a chosen place first, since two charts of the dashboard chose different places and both are kept");

pub fn wei_of(tokens: f64, rounding: WeiRounding) -> u128 {
    let scaled = (tokens * rounding.scale).round();
    match scaled <= none() {
        true => 0,
        false => (scaled as u128).saturating_mul(rounding.wei_per_scaled),
    }
}
because!(wei_of, "tokens as smallest units after rounding at the chosen place, the reverse of whole for a figure that goes back into the program's arithmetic");
