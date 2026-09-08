use crate::contexts::gate::state::{Guards, State};
use crate::contexts::gate::vocabulary::{Command, Event, Fault, Gate};
use patterns::{raise_by, refuse_when_lazily, Context, Guard};

impl Context for Gate {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> State {
        State { opened: 0, locked: false }
    }

    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Open => refuse_when_lazily(
                state,
                command,
                &[
                    (<State as Guards>::worn, Fault::Worn),
                    (<State as Guards>::shut, Fault::AlreadyLocked),
                ],
                Event::Opened,
            ),
            Command::Lock => {
                let guards: &[Guard<State, Command, Fault>] = &[
                    (<State as Guards>::shut, Fault::AlreadyLocked),
                    (<State as Guards>::worn, Fault::Worn),
                ];
                refuse_when_lazily(state, command, guards, Event::Locked)
            }
        }
    }

    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Opened => State { opened: raise_by(state.opened, 1), locked: state.locked },
            Event::Locked => State { opened: state.opened, locked: true },
        }
    }
}
