use patterns::Context;

pub struct State {
    pub open: bool,
}
pub struct Shared;
pub enum Command {
    Open,
    Close,
}
pub enum Event {
    Opened,
    Closed,
}
pub enum Fault {
    Stuck,
    Jammed,
}
impl Context for Shared {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;
    fn initial() -> State {
        State { open: false }
    }
    fn decide(_state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Open => Ok(Event::Opened),
            Command::Close => Ok(Event::Closed),
        }
    }
    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Opened => State { open: true },
            Event::Closed => State { open: state.open },
        }
    }
}
