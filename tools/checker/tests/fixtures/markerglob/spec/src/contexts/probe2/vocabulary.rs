use patterns::because;

pub struct Probe2;
because!(Probe2, "the probe context that tries to hold another context's state");
pub enum Event {
    Ticked,
}
because!(Event, "the one thing that happens in probe2");
