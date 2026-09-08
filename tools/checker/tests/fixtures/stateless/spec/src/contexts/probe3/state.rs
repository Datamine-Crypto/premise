use super::vocabulary::{Holder, Probe3};

pub struct State {
    pub n: u32,
}

impl Holder for Probe3 {
    type S = State;
    const START: State = State { n: 0 };
}
