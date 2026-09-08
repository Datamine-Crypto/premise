use patterns::{
    count_below, count_facet, count_from, found, handle, handle_all, holds, named, replay, size,
    value_of,
};
use spec::contexts::fines::state::{Loan, Member, State};
use spec::contexts::fines::vocabulary::{ByBorrower, ByCap, Command, Event, Fault, Fines, Grade};

pub fn current(events: &[Event]) -> State {
    replay::<Fines>(events)
}

pub fn step(state: State, command: &Command) -> Result<State, Fault> {
    handle::<Fines>(state, command)
}

pub fn steps(state: State, commands: &[Command]) -> Result<State, (usize, Fault)> {
    handle_all::<Fines>(state, commands)
}

pub fn enrolled(state: &State, member: u32) -> bool {
    holds(&state.members, member)
}

pub fn member_of(state: &State, member: u32) -> Option<Member> {
    found(&state.members, member)
}

pub fn out(state: &State, loan: u32) -> bool {
    holds(&state.loans, loan)
}

pub fn loan_of(state: &State, loan: u32) -> Option<Loan> {
    found(&state.loans, loan)
}

pub fn borrowed(state: &State, member: u32) -> usize {
    count_facet::<ByBorrower, Loan>(&state.loans, member)
}

pub fn lent(state: &State) -> usize {
    size(&state.loans)
}

pub fn ceiling(grade: &Grade) -> u32 {
    value_of::<ByCap, Grade>(grade)
}

pub fn grade_text(grade: &Grade) -> &'static str {
    named(grade)
}

pub fn overdue(state: &State) -> usize {
    count_below(&state.loans, state.today)
}

pub fn current_loans(state: &State) -> usize {
    count_from(&state.loans, state.today)
}
