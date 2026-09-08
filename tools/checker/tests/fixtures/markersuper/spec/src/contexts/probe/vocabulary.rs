use patterns::because;

pub struct Probe;
because!(Probe, "the probe context whose marker another context may try to drive");
pub enum Event {
    Ticked,
}
because!(Event, "the one thing that happens in the probe context");
