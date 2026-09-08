use crate::contexts::shared::vocabulary::*;
use patterns::Ranked;

pub struct State {
    pub n: u32,
    pub r: <Coin as Ranked>::Rank,
}
