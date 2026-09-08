#![forbid(unknown_lints)]
use patterns_macros::because;

pub const BLOCK_ONE_LITRES: u32 = 20;
because!(BLOCK_ONE_LITRES, "the free allowance the standing order grants each household");

pub const BLOCK_TWO_LITRES: u32 = 60;
because!(BLOCK_TWO_LITRES, "where the standing order starts charging the higher rate");
