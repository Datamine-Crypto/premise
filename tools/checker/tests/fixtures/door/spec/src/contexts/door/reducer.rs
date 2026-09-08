use crate::contexts::door::state::State;
use crate::contexts::door::vocabulary::{Command, Door, Event, Fault, OPEN_LIMIT};
use patterns::{either, is_at_least, raise_by, Context};

impl Context for Door {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> State {
        State { opened: 0, locked: false }
    }

    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Open => either(
                is_at_least(state.opened, OPEN_LIMIT),
                Err(Fault::Worn),
                Ok(Event::Opened),
            ),
            Command::Lock => either(state.locked, Err(Fault::AlreadyLocked), Ok(Event::Locked)),
        }
    }

    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Opened => State { opened: raise_by(state.opened, 1), locked: state.locked },
            Event::Locked => State { opened: state.opened, locked: true },
        }
    }
}
