use patterns::{because, decided};

pub struct Probe;
because!(Probe, "the probe context that carries a floor and a ceiling");
pub trait Cap {
    const LO: u32;
    const HI: u32;
}
decided!(Probe, Cap, "the floor and the ceiling the desk settled on for the probe context");
impl Cap for Probe {
    const LO: u32 = 15;
    const HI: u32 = 500;
}
