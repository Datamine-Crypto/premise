use patterns::{because, decided};

pub struct Probe;
because!(Probe, "the probe context that carries a cap of its own");
pub trait Cap {
    const CAP: u32;
}
decided!(Probe, Cap, "the cap the desk settled on for the probe context");
impl Cap for Probe {
    const CAP: u32 = 500;
}
