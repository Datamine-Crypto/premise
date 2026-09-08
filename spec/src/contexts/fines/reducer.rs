use crate::contexts::fines::state::{Charge, Loan, Member, Settle, State};
use crate::contexts::fines::vocabulary::{
    ByCap, ByCharge, BySettle, Command, Event, Fault, Fines, Grade, BORROW_BAR_PENCE,
    CHARGES_DAY, LEGACY_PENCE_PER_DAY, LOAN_DAYS, PENCE_PER_DAY,
};
use patterns::{
    clamp_upper, either, found, holds, is_above, is_below, is_same, mul, raise_by, reduce_by,
    refuse_when, shift_all, value_of, with, without, Context,
};

impl Context for Fines {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> State {
        State {
            members: vec![],
            loans: vec![],
            today: 0,
        }
    }

    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Enrol { member, grade } => either(
                holds(&state.members, *member),
                Err(Fault::AlreadyEnrolled),
                Ok(Event::Enrolled {
                    member: *member,
                    grade: *grade,
                }),
            ),
            Command::Borrow { loan, member } => {
                let who = found(&state.members, *member);
                match who {
                    Some(one) => refuse_when(
                        &[
                            (holds(&state.loans, *loan), Fault::AlreadyOnLoan),
                            (is_above(one.owed, BORROW_BAR_PENCE), Fault::Barred),
                        ],
                        Event::Borrowed {
                            loan: *loan,
                            member: *member,
                            due: raise_by(state.today, LOAN_DAYS),
                        },
                    ),
                    None => Err(Fault::NotEnrolled),
                }
            }
            Command::Return { loan } => {
                let out = found(&state.loans, *loan);
                match out {
                    Some(one) => {
                        let who = found(&state.members, one.member);
                        match who {
                            Some(borrower) => Ok(Event::Returned {
                                loan: *loan,
                                member: one.member,
                                fine: clamp_upper(
                                    mul(
                                        reduce_by(state.today, one.due),
                                        either(
                                            is_below(one.due, CHARGES_DAY),
                                            LEGACY_PENCE_PER_DAY,
                                            PENCE_PER_DAY,
                                        ),
                                    ),
                                    value_of::<ByCap, Grade>(&borrower.grade),
                                ),
                            }),
                            None => Err(Fault::NotEnrolled),
                        }
                    }
                    None => Err(Fault::NoLoan),
                }
            }
            Command::Pay { member, amount } => {
                let who = found(&state.members, *member);
                match who {
                    Some(one) => either(
                        is_same(one.owed, 0),
                        Err(Fault::NothingOwed),
                        Ok(Event::Paid {
                            member: *member,
                            amount: clamp_upper(*amount, one.owed),
                        }),
                    ),
                    None => Err(Fault::NotEnrolled),
                }
            }
            Command::PassDay => Ok(Event::DayPassed),
        }
    }

    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Enrolled { member, grade } => State {
                members: with(
                    &state.members,
                    Member {
                        id: *member,
                        grade: *grade,
                        owed: 0,
                    },
                ),
                loans: state.loans,
                today: state.today,
            },
            Event::Borrowed { loan, member, due } => State {
                members: state.members,
                loans: with(
                    &state.loans,
                    Loan {
                        id: *loan,
                        member: *member,
                        due: *due,
                    },
                ),
                today: state.today,
            },
            Event::Returned { loan, member, fine } => State {
                members: shift_all::<ByCharge, Member>(
                    &state.members,
                    Charge {
                        member: *member,
                        fine: *fine,
                    },
                ),
                loans: without(&state.loans, *loan),
                today: state.today,
            },
            Event::Paid { member, amount } => State {
                members: shift_all::<BySettle, Member>(
                    &state.members,
                    Settle {
                        member: *member,
                        amount: *amount,
                    },
                ),
                loans: state.loans,
                today: state.today,
            },
            Event::DayPassed => State {
                members: state.members,
                loans: state.loans,
                today: raise_by(state.today, 1),
            },
        }
    }
}
