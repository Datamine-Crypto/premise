use patterns::{
    any_facet, count_above, found, handle, handle_all, holds, named, replay, size, value_of,
};
use spec::contexts::seasonticket::state::{State, Ticket};
use spec::contexts::seasonticket::vocabulary::{
    ByBay, ByStandDown, Command, Cover, Event, Fault, SeasonTicket, Tier,
};

pub fn current(events: &[Event]) -> State {
    replay::<SeasonTicket>(events)
}

pub fn step(state: State, command: &Command) -> Result<State, Fault> {
    handle::<SeasonTicket>(state, command)
}

pub fn steps(state: State, commands: &[Command]) -> Result<State, (usize, Fault)> {
    handle_all::<SeasonTicket>(state, commands)
}

pub fn issued(state: &State, ticket: u32) -> bool {
    holds(&state.tickets, ticket)
}

pub fn ticket_of(state: &State, ticket: u32) -> Option<Ticket> {
    found(&state.tickets, ticket)
}

pub fn let_out(state: &State, bay: u32) -> bool {
    any_facet::<ByBay, Ticket>(&state.tickets, bay)
}

pub fn running(state: &State) -> usize {
    count_above(&state.tickets, state.today)
}

pub fn sold(state: &State) -> usize {
    size(&state.tickets)
}

pub fn allowance(tier: &Tier) -> u32 {
    value_of::<ByStandDown, Tier>(tier)
}

pub fn tier_text(tier: &Tier) -> &'static str {
    named(tier)
}

pub fn cover_text(cover: &Cover) -> &'static str {
    named(cover)
}
