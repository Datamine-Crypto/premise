use patterns::{any_facet, count_facet, handle, handle_all, holds, replay, size, value_of};
use spec::contexts::holdqueue::state::{Hold, State};
use spec::contexts::holdqueue::vocabulary::{
    ByAllowance, ByClaim, ByMember, ByQueue, Command, Event, Fault, HoldQueue, Ready, Standing,
};

pub fn current(events: &[Event]) -> State {
    replay::<HoldQueue>(events)
}

pub fn step(state: State, command: &Command) -> Result<State, Fault> {
    handle::<HoldQueue>(state, command)
}

pub fn steps(state: State, commands: &[Command]) -> Result<State, (usize, Fault)> {
    handle_all::<HoldQueue>(state, commands)
}

pub fn queued(state: &State, member: u32) -> usize {
    count_facet::<ByMember, Hold>(&state.queue, member)
}

pub fn allowance(standing: &Standing) -> usize {
    value_of::<ByAllowance, Standing>(standing)
}

pub fn at_stage(state: &State, title: u32, stage: Ready) -> bool {
    any_facet::<ByQueue, Hold>(&state.queue, (title, stage))
}

pub fn depth(state: &State) -> usize {
    size(&state.queue)
}

pub fn queuing(state: &State, title: u32, member: u32) -> bool {
    holds(&state.queue, (title, member))
}

pub fn claimed(state: &State, title: u32, member: u32) -> bool {
    any_facet::<ByClaim, Hold>(&state.queue, (title, member, Ready::Notified))
}
