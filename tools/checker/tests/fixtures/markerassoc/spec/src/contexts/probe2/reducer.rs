use super::state::State;
use super::vocabulary::{Event, Probe2};
use crate::contexts::shared::vocabulary::*;
use patterns::Context;

impl Context for Probe2 {
    type State = State;
    type Event = Event;
    fn initial() -> State {
        State { n: 0 }
    }
    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Ticked => state,
        }
    }
}
