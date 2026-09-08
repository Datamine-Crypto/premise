use super::state::State;
use super::vocabulary::{Event, Probe2};
use patterns::Context;

impl Context for Probe2 {
    type State = State;
    type Event = Event;
    fn initial() -> State {
        State { n: 0 }
    }
    fn apply(state: State, event: &Event) -> State {
        let replay = state.n;
        let handle = replay;
        match event {
            Event::Ticked => State { n: handle },
        }
    }
}
