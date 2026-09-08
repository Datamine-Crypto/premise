use patterns::because;

pub struct Probe2;
because!(Probe2, "the probe context that tries to hold another context's state");
pub enum Event {
    Ticked,
}
because!(Event, "the one thing that happens in probe2");
use crate::contexts::probe::vocabulary::Probe;
use patterns::decided;

pub trait Rule {
    const X: u32;
}

decided!(Probe, Rule, "a fact about the probe context that probe2 decided on its behalf");

impl Rule for Probe {
    const X: u32 = 5;
}
