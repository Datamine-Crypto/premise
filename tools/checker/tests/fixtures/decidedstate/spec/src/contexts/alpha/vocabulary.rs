use patterns::because;

pub struct Probe;
because!(Probe, "the probe context whose floor is set in its state file");
pub trait Floor {
    const FLOOR: u32;
}
because!(Floor, "the least a probe may read before it is judged empty");
