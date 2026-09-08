use patterns::{because, decided};

pub struct Probe;
because!(Probe, "the probe context that carries a bar and a cap under one trait");
pub struct ByBar;
pub struct ByCap;
pub trait Cap<M> {
    const CAP: u32;
}
because!(Cap, "a ceiling keyed by the marker that says which ceiling");
decided!(Probe, Cap<ByBar>, "what the desk settled on after the spring count of bays");
impl Cap<ByBar> for Probe {
    const CAP: u32 = 250;
}
decided!(Probe, Cap<ByCap>, "what the desk settled on after the autumn count of bays");
impl Cap<ByCap> for Probe {
    const CAP: u32 = 500;
}
