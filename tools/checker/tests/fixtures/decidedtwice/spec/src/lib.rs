use patterns::{because, decided};

pub struct Probe;
because!(Probe, "the probe context that carries a bar and a cap of its own");
pub trait Bar {
    const BAR: u32;
}
pub trait Cap {
    const CAP: u32;
}
decided!(Probe, Bar, "the bar the desk settled on for the probe context");
impl Bar for Probe {
    const BAR: u32 = 250;
}
decided!(Probe, Cap, "the cap the desk settled on for the probe context");
impl Cap for Probe {
    const CAP: u32 = 500;
}
