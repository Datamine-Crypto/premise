use patterns::{
    any_facet, count_facet, found, handle, handle_all, holds, is_full, mul, replay, size, value_of,
};
use spec::contexts::lockers::state::{Rental, Salvage, State};
use spec::contexts::lockers::vocabulary::{
    ByAbandoned, ByContents, ByHolder, ByRate, ByRenter, BySize, Command, Contents, Event, Fault,
    Lockers, Moved, Size, LOCKER_BAYS, LOCKER_TERM_DAYS,
};

pub fn current(events: &[Event]) -> State {
    replay::<Lockers>(events)
}

pub fn step(state: State, command: &Command) -> Result<State, Fault> {
    handle::<Lockers>(state, command)
}

pub fn steps(state: State, commands: &[Command]) -> Result<State, (usize, Fault)> {
    handle_all::<Lockers>(state, commands)
}

pub fn rate(size: &Size) -> u32 {
    value_of::<ByRate, Size>(size)
}

pub fn fee(size: &Size) -> u32 {
    mul(value_of::<ByRate, Size>(size), LOCKER_TERM_DAYS)
}

pub fn rented(state: &State, locker: u32) -> bool {
    holds(&state.rentals, locker)
}

pub fn letting(state: &State, locker: u32) -> Option<Rental> {
    found(&state.rentals, locker)
}

pub fn held_by(state: &State, locker: u32, renter: u32) -> bool {
    any_facet::<ByRenter, Rental>(&state.rentals, (locker, renter))
}

pub fn transferable(state: &State, locker: u32, renter: u32) -> bool {
    any_facet::<ByHolder, Rental>(&state.rentals, (locker, renter, Moved::Held))
}

pub fn abandoned(state: &State, locker: u32) -> bool {
    any_facet::<ByAbandoned, Rental>(&state.rentals, (locker, 0))
}

pub fn out(state: &State, which: Size) -> usize {
    count_facet::<BySize, Rental>(&state.rentals, which)
}

pub fn banked(state: &State) -> usize {
    size(&state.rentals)
}

pub fn reclaimed(state: &State, contents: Contents) -> usize {
    count_facet::<ByContents, Salvage>(&state.log, contents)
}

pub fn salvage_of(state: &State, locker: u32) -> Option<Salvage> {
    found(&state.log, locker)
}

pub fn full(state: &State) -> bool {
    is_full(&state.rentals, LOCKER_BAYS)
}
