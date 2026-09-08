use patterns::{because, Context};

pub struct Probe2;
because!(Probe2, "the probe context that tries to hold another context's state");
pub enum Event {
    Ticked,
}
because!(Event, "the one thing that happens in probe2");
pub trait Machine: Context {}

impl Machine for crate::contexts::probe::vocabulary::Probe {}

pub struct Holds<M: Machine>(pub M::State);
