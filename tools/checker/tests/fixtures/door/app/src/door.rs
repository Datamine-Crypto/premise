use patterns::{handle, is_at_least, replay};
use spec::contexts::door::state::State;
use spec::contexts::door::vocabulary::{Command, Door, Event, Fault, OPEN_LIMIT};

pub fn current(events: &[Event]) -> State {
    replay::<Door>(events)
}

pub fn step(state: State, command: &Command) -> Result<State, Fault> {
    handle::<Door>(state, command)
}

pub fn worn(state: &State) -> bool {
    is_at_least(state.opened, OPEN_LIMIT)
}
