use patterns_macros::because;

pub trait Context {
    type State;
    type Command;
    type Event;
    type Fault;

    fn initial() -> Self::State;
    fn decide(state: &Self::State, command: &Self::Command) -> Result<Self::Event, Self::Fault>;
    fn apply(state: Self::State, event: &Self::Event) -> Self::State;
}

pub fn replay_from<C: Context>(state: C::State, events: &[C::Event]) -> C::State {
    let mut next = state;
    for event in events {
        next = C::apply(next, event);
    }
    next
}

pub fn replay<C: Context>(events: &[C::Event]) -> C::State {
    replay_from::<C>(C::initial(), events)
}

pub fn handle<C: Context>(state: C::State, command: &C::Command) -> Result<C::State, C::Fault> {
    match C::decide(&state, command) {
        Ok(event) => Ok(C::apply(state, &event)),
        Err(fault) => Err(fault),
    }
}

pub fn handle_all<C: Context>(
    state: C::State,
    commands: &[C::Command],
) -> Result<C::State, (usize, C::Fault)> {
    let mut next = state;
    for (at, command) in commands.iter().enumerate() {
        match handle::<C>(next, command) {
            Ok(moved) => next = moved,
            Err(fault) => return Err((at, fault)),
        }
    }
    Ok(next)
}
because!(replay_from, "applying a run of events to a state that already exists, which is how replay works forward from initial and how a caller holding a snapshot catches up without folding the whole log again");
because!(replay, "folding a whole event log into the current state, so state is always derived and never stored");
because!(handle, "deciding on a command and applying the resulting event, so a caller cannot apply an event the rules did not produce");
because!(handle_all, "handle over a sequence, stopping at the first fault and naming the position of the command that raised it, because a partially applied batch is a state no replay can reach and a caller can only report or retry a batch when it knows where it broke");
