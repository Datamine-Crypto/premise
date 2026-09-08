use patterns_macros::{because, source};
use std::collections::BTreeMap;

pub fn whole_tokens(wei: u128, wei_per_unit: u128, units_per_token: f64) -> f64 {
    match wei_per_unit == 0 {
        true => wei as f64,
        false => ((wei / wei_per_unit) as f64) / units_per_token,
    }
}
because!(whole_tokens, "an amount in smallest units as a number of whole tokens, truncated to the units the caller keeps before the division, so the figure equals what the retired dashboard showed to the place");

pub struct DecimalNotation;
source!(DecimalNotation, "the base ten positional notation every shown number is written in, which fixes what one more place of precision multiplies by");

pub struct RoundHalfUp;
source!(RoundHalfUp, "the rounding the dashboard runtime applies, where a value exactly between two neighbours goes to the larger one");

const DECIMAL_BASE: f64 = 10.0;
because!(DECIMAL_BASE, DecimalNotation, "what a number is scaled by for each place it is rounded to");

const HALF: f64 = 0.5;
because!(HALF, RoundHalfUp, "what is added before the floor so that a value halfway between two neighbours lands on the larger, the rule the runtime follows");

pub fn rounded(value: f64, places: u32) -> f64 {
    let scale = DECIMAL_BASE.powi(places as i32);
    (value * scale + HALF).floor() / scale
}
because!(rounded, "a number to a fixed count of places, rounding half toward the larger value the way the retired dashboard's runtime rounds, so a figure shown here matches one shown there");

pub fn percent_of(part: f64, whole: f64, percent: f64) -> Option<f64> {
    let none = f64::from(0u8);
    match whole > none {
        true => Some(part / whole * percent),
        false => None,
    }
}
because!(percent_of, "a part as a share of a whole in percent, or nothing when the whole is empty, since a share of nothing is not zero but undefined and a chart should skip the day");

pub fn ratio_of(part: f64, whole: f64) -> Option<f64> {
    let none = f64::from(0u8);
    match whole > none {
        true => Some(part / whole),
        false => None,
    }
}
because!(ratio_of, "a part over a whole, or nothing when the whole is empty, the unscaled form of percent_of for a ratio shown as a plain number");

pub fn priced(amount: f64, price: f64) -> f64 {
    amount * price
}
because!(priced, "an amount at a price, the multiply a binding may not write as an operator");

pub fn divided(value: f64, by: f64) -> Option<f64> {
    let none = f64::from(0u8);
    match by == none {
        true => None,
        false => Some(value / by),
    }
}
because!(divided, "a number over another, or nothing when the divisor is zero, the divide a binding may not write and a reader may not crash on");

pub fn added(a: f64, b: f64) -> f64 {
    a + b
}
because!(added, "two numbers as one, the add a binding may not write as an operator, for running sums in dollars");

pub fn taken(a: f64, b: f64) -> f64 {
    a - b
}
because!(taken, "one number less another, the subtract a binding may not write as an operator, for a net of two dollar sums");

pub fn is_positive(value: f64) -> bool {
    value > f64::from(0u8)
}
because!(is_positive, "whether a number is above zero, asked as a call, the guard before a division or a chart point");

pub fn is_finite_positive(value: f64) -> bool {
    value.is_finite() && value > f64::from(0u8)
}
because!(is_finite_positive, "whether a number is a real value above zero, the guard a chart point passes before it is drawn, since a division that overflowed is not a point");

pub fn as_number(value: u64) -> f64 {
    value as f64
}
because!(as_number, "a count as a floating number so it can be divided or scaled, the cast a binding may not write");

pub trait Earner {
    type Who: Ord + Clone;
    fn who(&self) -> Self::Who;
    fn moment(&self) -> u64;
    fn amount(&self) -> u128;
    fn earns_minted(&self) -> bool;
    fn earns_credit(&self) -> bool;
}
because!(Earner, "how a ledger row shows the dollar walk who acted, when, how much, and which of two sums the row belongs to, so the walk prices rows it never defined");

#[derive(patterns_macros::Record)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Dollars {
    pub minted_usd: f64,
    pub credit_usd: f64,
}
because!(Dollars, "what one account earned in dollars at the prices of the days it earned them: what it minted and the burn credit it was given");

pub fn walked_usd<E: Earner>(
    totals: BTreeMap<E::Who, Dollars>,
    rows: &[E],
    prices: &BTreeMap<u64, f64>,
    first_price: Option<f64>,
    day_seconds: u64,
    wei_per_unit: u128,
    units_per_token: f64,
) -> BTreeMap<E::Who, Dollars> {
    let mut out = totals;
    let none = f64::from(0u8);
    for row in rows {
        let day = crate::day::day_of(row.moment(), day_seconds);
        let price = prices
            .range(..=day)
            .next_back()
            .map(|(_, p)| *p)
            .or(first_price)
            .unwrap_or(none);
        let usd = whole_tokens(row.amount(), wei_per_unit, units_per_token) * price;
        let earned = out.entry(row.who()).or_default();
        if row.earns_minted() {
            earned.minted_usd += usd;
        }
        if row.earns_credit() {
            earned.credit_usd += usd;
        }
    }
    out
}
because!(walked_usd, "ledger rows priced at the day price carried forward to each row's day, falling back to the first known price for rows older than the price series rather than to nothing, and summed into each account's two dollar figures");

pub fn none() -> f64 {
    f64::from(0u8)
}
because!(none, "zero as a floating number, the comparison floor every chart tests a denominator against, written as a call because a floating literal is a value from nowhere");

pub fn unit() -> f64 {
    none().exp()
}
because!(unit, "one as a floating number, the exponential of zero, which is exactly one, written as a call because a float literal in a pattern is a fact from nowhere");

pub fn yield_factor(blocks_per_day: u64, per_block_divisor: u128, year_days: u64, time_steps: u128, percent: f64) -> f64 {
    blocks_per_day as f64 * (unit() / per_block_divisor as f64) * year_days as f64 * time_steps as f64 * percent
}
because!(yield_factor, "the yearly yield with no burn as a percent of the stake, a day of blocks at the base rate for a year at the top time multiplier, multiplied in the order the dashboard multiplied so the floating figure is the same one");
