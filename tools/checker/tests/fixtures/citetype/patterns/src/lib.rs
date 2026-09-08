use core::ops::Add;
use patterns_macros::because;

const GAMMA: u64 = 7;
because!(GAMMA, Add, "a constant that cites a trait rather than a source");
