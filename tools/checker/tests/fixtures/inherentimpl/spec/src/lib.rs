use patterns::because;

pub struct Probe;
because!(Probe, "the probe fixture, which carries two ceilings on itself");
impl Probe {
    pub const MAX: u32 = 500;
    pub const MIN: u32 = 7;
}
