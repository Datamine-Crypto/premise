use crate::contexts::lockers::state::{Guards, Rental, Salvage, State};
use crate::contexts::lockers::vocabulary::{
    ByAbandoned, ByHolder, ByRenter, Command, Event, Fault, Lockers, Moved, LOCKER_GRACE_DAYS,
    LOCKER_TERM_DAYS,
};
use patterns::{
    advance_all, any_facet, either, found, holds, refuse_when_lazily, with, without, Context, Guard,
};

impl Context for Lockers {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> State {
        State {
            rentals: vec![],
            log: vec![],
        }
    }

    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Rent {
                locker,
                renter,
                size,
            } => {
                let guards: &[Guard<State, Command, Fault>] = &[
                    (<State as Guards>::taken, Fault::AlreadyRented),
                    (<State as Guards>::full, Fault::Full),
                ];
                refuse_when_lazily(
                    state,
                    command,
                    guards,
                    Event::Rented {
                        locker: *locker,
                        renter: *renter,
                        size: *size,
                    },
                )
            }
            Command::Transfer { locker, from, to } => {
                let letting = found(&state.rentals, *locker);
                either(
                    any_facet::<ByHolder, Rental>(&state.rentals, (*locker, *from, Moved::Held)),
                    match letting {
                        Some(rental) => Ok(Event::Transferred {
                            locker: *locker,
                            renter: *to,
                            size: rental.size,
                            left: rental.left,
                        }),
                        None => Err(Fault::NotRented),
                    },
                    either(
                        any_facet::<ByRenter, Rental>(&state.rentals, (*locker, *from)),
                        Err(Fault::AlreadyTransferred),
                        either(
                            holds(&state.rentals, *locker),
                            Err(Fault::NotTheRenter),
                            Err(Fault::NotRented),
                        ),
                    ),
                )
            }
            Command::Reclaim { locker, contents } => {
                let letting = found(&state.rentals, *locker);
                match letting {
                    Some(rental) => either(
                        any_facet::<ByAbandoned, Rental>(&state.rentals, (*locker, 0)),
                        Ok(Event::Reclaimed {
                            locker: *locker,
                            renter: rental.renter,
                            contents: *contents,
                        }),
                        Err(Fault::WithinTerm),
                    ),
                    None => Err(Fault::NotRented),
                }
            }
            Command::PassDay => Ok(Event::DayPassed),
        }
    }

    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Rented {
                locker,
                renter,
                size,
            } => State {
                rentals: with(
                    &state.rentals,
                    Rental {
                        locker: *locker,
                        renter: *renter,
                        size: *size,
                        left: LOCKER_TERM_DAYS,
                        grace: LOCKER_GRACE_DAYS,
                        moved: Moved::Held,
                    },
                ),
                log: state.log,
            },
            Event::Transferred {
                locker,
                renter,
                size,
                left,
            } => State {
                rentals: with(
                    &without(&state.rentals, *locker),
                    Rental {
                        locker: *locker,
                        renter: *renter,
                        size: *size,
                        left: *left,
                        grace: LOCKER_GRACE_DAYS,
                        moved: Moved::Passed,
                    },
                ),
                log: state.log,
            },
            Event::Reclaimed {
                locker,
                renter,
                contents,
            } => State {
                rentals: without(&state.rentals, *locker),
                log: with(
                    &state.log,
                    Salvage {
                        locker: *locker,
                        renter: *renter,
                        contents: *contents,
                    },
                ),
            },
            Event::DayPassed => State {
                rentals: advance_all(&state.rentals, 1),
                log: state.log,
            },
        }
    }
}
