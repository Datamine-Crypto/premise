use patterns::{because, raise_by};

pub struct Probe;
because!(Probe, "the probe fixture whose one rule the binding below reaches");
pub const PENCE: u32 = 1;
because!(PENCE, "the one unit the probe rule adds on every run");
pub trait Rule {
    fn run(n: u32) -> u32;
}
impl Rule for Probe {
    fn run(n: u32) -> u32 {
        raise_by(n, PENCE)
    }
}
