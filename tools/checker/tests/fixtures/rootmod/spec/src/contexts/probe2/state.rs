use patterns::Context;

mod core {
    pub use crate::contexts::probe::vocabulary::Probe;
}

type FS = <core::Probe as Context>::State;

pub struct State {
    pub a: Option<FS>,
}
