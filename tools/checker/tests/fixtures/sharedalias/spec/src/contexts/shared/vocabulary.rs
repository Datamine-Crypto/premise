use patterns::Context as Ctx;

pub struct State(pub u32);
pub struct Meter;
pub enum Command {
    Tick,
    Stop,
}
pub enum Event {
    Ticked,
    Stopped,
}
pub enum Fault {
    Stuck,
    Jammed,
}
impl Ctx for Meter {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;
    fn initial() -> State {
        State(0)
    }
    fn decide(_state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Tick => Ok(Event::Ticked),
            Command::Stop => Ok(Event::Stopped),
        }
    }
    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Ticked => State(state.0),
            Event::Stopped => State(state.0),
        }
    }
}
