use crate::contexts::probe::vocabulary::Probe;
use patterns::{replay, Context};

pub struct State {
    pub probe: <Probe as Context>::State,
}

impl State {
    fn fresh() -> State {
        State { probe: replay::<Probe>(&[]) }
    }
}
