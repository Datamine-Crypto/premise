use patterns::because;

pub struct Probe2;
because!(Probe2, "the probe context that tries to hold another context's state");
pub enum Event {
    Ticked,
}
because!(Event, "the one thing that happens in probe2");
use crate::contexts::probe3::vocabulary::Holder;

pub struct Holds2<T: Holder>(pub T::S);
