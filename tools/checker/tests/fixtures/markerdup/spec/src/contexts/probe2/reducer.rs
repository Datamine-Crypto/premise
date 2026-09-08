use super::state::State;
use super::vocabulary::{Event, Probe, Probe2};
use patterns::Context;

impl Context for Probe2 {
    type State = State;
    type Event = Event;
    fn initial() -> State {
        State { n: 0 }
    }
    fn apply(state: State, _event: &Event) -> State {
        state
    }
}

impl Context for Probe {
    type State = u32;
    type Event = Event;
    fn initial() -> u32 {
        0
    }
    fn apply(state: u32, _event: &Event) -> u32 {
        state
    }
}
