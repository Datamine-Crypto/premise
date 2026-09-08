use spec::{Probe, Rule};

pub fn ran(n: u32) -> u32 {
    <Probe as Rule>::run(n)
}
