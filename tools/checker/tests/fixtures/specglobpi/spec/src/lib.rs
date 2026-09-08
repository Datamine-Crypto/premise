use patterns::{because, Ranked};
use std::f64::consts::*;

pub struct Probe;
because!(Probe, "the probe fixture whose rank is a circle constant from outside");
impl Ranked for Probe {
    type Rank = f64;
    fn rank(&self) -> f64 {
        PI
    }
}
