use super::state::{Guards, State};
use super::vocabulary::{Event, Probe2};
use patterns::Context;

impl Context for Probe2 {
    type State = State;
    type Event = Event;
    fn initial() -> State {
        State { n: 0 }
    }
    fn apply(state: State, event: &Event) -> State {
        let _checks: &[fn(&State) -> bool] = &[<State as Guards>::full];
        match event {
            Event::Ticked => state,
        }
    }
}
