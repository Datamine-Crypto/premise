use patterns::{at_or_before, none, whole, Prices, Units};
use patterns_macros::because;
use std::collections::BTreeMap;
use ruint::aliases::U256;

pub fn virtual_reserves(liquidity: u128, sqrt_price: u128, fraction_bits: u32) -> (u128, u128) {
    let scale = U256::from(1u8) << fraction_bits;
    let l = U256::from(liquidity);
    let p = U256::from(sqrt_price);
    match p.is_zero() {
        true => (0, 0),
        false => {
            let first = (l * scale) / p;
            let second = (l * p) / scale;
            (first.saturating_to(), second.saturating_to())
        }
    }
}
because!(virtual_reserves, "the two reserves a concentrated pool would hold at its reported price and liquidity, computed wide because the liquidity times the price scale passes the widest primitive, and the division truncating as the pool's own arithmetic does");

pub fn reserve_price(token_reserve: f64, other_reserve: f64) -> Option<f64> {
    let none = f64::from(0u8);
    match token_reserve > none && other_reserve > none {
        true => Some(other_reserve / token_reserve),
        false => None,
    }
}
because!(reserve_price, "the price of a token in the pool's other side from the two reserves, or nothing when either side is empty, since an empty pool has no price and a zero would read as one");

pub fn day_prices(reserves: &BTreeMap<u64, (u128, u128)>, eth: &Prices, units: Units) -> Prices {
    let mut out = BTreeMap::new();
    for (day, (token, other)) in reserves {
        let eth_price = match at_or_before(eth, *day) {
            Some(p) => p,
            None => continue,
        };
        let token_whole = whole(*token, units);
        let other_whole = whole(*other, units);
        if token_whole <= none() || other_whole <= none() {
            continue;
        }
        out.insert(*day, (other_whole / token_whole) * eth_price);
    }
    out
}
because!(day_prices, "a token's dollar price on each day its pool reported, the pool's ratio of the currency side to the token side at that day's carried forward currency price, skipping days with no currency price or an empty side");
