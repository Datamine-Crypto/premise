use crate::contexts::seasonticket::state::{State, Step, Ticket};
use crate::contexts::seasonticket::vocabulary::{
    ByBay, ByStandDown, Command, Cover, Event, Fault, SeasonTicket, Tier, LEGACY_REASSIGN_PENCE,
    REASSIGN_PENCE, TARIFF_DAY, TERM_DAYS,
};
use patterns::{
    advance_all, any_facet, clamp_upper, either, found, holds, is_at_least, is_below, is_same,
    raise_by, reduce_by, value_of, with, without, Context,
};

impl Context for SeasonTicket {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> State {
        State {
            tickets: vec![],
            today: 0,
        }
    }

    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Buy { ticket, bay, tier } => either(
                holds(&state.tickets, *ticket),
                Err(Fault::AlreadyIssued),
                either(
                    any_facet::<ByBay, Ticket>(&state.tickets, *bay),
                    Err(Fault::BayTaken),
                    Ok(Event::Bought {
                        ticket: *ticket,
                        bay: *bay,
                        tier: *tier,
                    }),
                ),
            ),
            Command::Suspend { ticket } => {
                let held = found(&state.tickets, *ticket);
                match held {
                    Some(one) => either(
                        is_same(one.cover, Cover::Suspended),
                        Err(Fault::AlreadySuspended),
                        either(
                            is_at_least(state.today, one.expires),
                            Err(Fault::Lapsed),
                            either(
                                is_at_least(one.used, value_of::<ByStandDown, Tier>(&one.tier)),
                                Err(Fault::AllowanceSpent),
                                Ok(Event::Suspended {
                                    ticket: *ticket,
                                    since: state.today,
                                }),
                            ),
                        ),
                    ),
                    None => Err(Fault::NoTicket),
                }
            }
            Command::Resume { ticket } => {
                let held = found(&state.tickets, *ticket);
                match held {
                    Some(one) => either(
                        is_same(one.cover, Cover::Active),
                        Err(Fault::NotSuspended),
                        Ok(Event::Resumed {
                            ticket: *ticket,
                            granted: clamp_upper(
                                reduce_by(state.today, one.since),
                                reduce_by(value_of::<ByStandDown, Tier>(&one.tier), one.used),
                            ),
                        }),
                    ),
                    None => Err(Fault::NoTicket),
                }
            }
            Command::Reassign { ticket, bay } => {
                let held = found(&state.tickets, *ticket);
                match held {
                    Some(one) => either(
                        is_same(one.bay, *bay),
                        Err(Fault::SameBay),
                        either(
                            is_at_least(state.today, one.expires),
                            Err(Fault::Lapsed),
                            either(
                                any_facet::<ByBay, Ticket>(&state.tickets, *bay),
                                Err(Fault::BayTaken),
                                Ok(Event::Reassigned {
                                    ticket: *ticket,
                                    bay: *bay,
                                    fee: either(
                                        is_below(one.bought, TARIFF_DAY),
                                        LEGACY_REASSIGN_PENCE,
                                        REASSIGN_PENCE,
                                    ),
                                }),
                            ),
                        ),
                    ),
                    None => Err(Fault::NoTicket),
                }
            }
            Command::Surrender { ticket } => either(
                holds(&state.tickets, *ticket),
                Ok(Event::Surrendered { ticket: *ticket }),
                Err(Fault::NoTicket),
            ),
            Command::PassDay => Ok(Event::DayPassed),
        }
    }

    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Bought { ticket, bay, tier } => State {
                today: state.today,
                tickets: with(
                    &state.tickets,
                    Ticket {
                        id: *ticket,
                        bay: *bay,
                        tier: *tier,
                        bought: state.today,
                        expires: raise_by(state.today, TERM_DAYS),
                        used: 0,
                        since: 0,
                        cover: Cover::Active,
                        owed: 0,
                    },
                ),
            },
            Event::Suspended { ticket, since } => State {
                today: state.today,
                tickets: advance_all(
                    &state.tickets,
                    Step::Hold {
                        ticket: *ticket,
                        since: *since,
                    },
                ),
            },
            Event::Resumed { ticket, granted } => State {
                today: state.today,
                tickets: advance_all(
                    &state.tickets,
                    Step::Release {
                        ticket: *ticket,
                        granted: *granted,
                    },
                ),
            },
            Event::Reassigned { ticket, bay, fee } => State {
                today: state.today,
                tickets: advance_all(
                    &state.tickets,
                    Step::Move {
                        ticket: *ticket,
                        bay: *bay,
                        fee: *fee,
                    },
                ),
            },
            Event::Surrendered { ticket } => State {
                today: state.today,
                tickets: without(&state.tickets, *ticket),
            },
            Event::DayPassed => State {
                today: raise_by(state.today, 1),
                tickets: state.tickets,
            },
        }
    }
}
