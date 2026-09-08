use crate::contexts::holdqueue::state::{Hold, State};
use crate::contexts::holdqueue::vocabulary::{
    ByAllowance, ByClaim, ByMember, ByQueue, Command, Event, Fault, HoldQueue, Ready, Standing,
    COLLECT_DAYS,
};
use patterns::{
    advance_all, any_facet, count_facet, either, first_facet, holds, is_at_least, keep_above,
    value_of, with, without, Context,
};

impl Context for HoldQueue {
    type State = State;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> State {
        State { queue: vec![] }
    }

    fn decide(state: &State, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Place {
                title,
                member,
                standing,
            } => either(
                holds(&state.queue, (*title, *member)),
                Err(Fault::AlreadyQueued),
                either(
                    is_at_least(
                        count_facet::<ByMember, Hold>(&state.queue, *member),
                        value_of::<ByAllowance, Standing>(standing),
                    ),
                    Err(Fault::AtAllowance),
                    Ok(Event::Placed {
                        title: *title,
                        member: *member,
                    }),
                ),
            ),
            Command::Available { title } => {
                let next = first_facet::<ByQueue, Hold>(&state.queue, (*title, Ready::Waiting));
                either(
                    any_facet::<ByQueue, Hold>(&state.queue, (*title, Ready::Notified)),
                    Err(Fault::AlreadyOffered),
                    match next {
                        Some(hold) => Ok(Event::Notified {
                            title: *title,
                            member: hold.member,
                        }),
                        None => Err(Fault::NoQueue),
                    },
                )
            }
            Command::Collect { title, member } => either(
                any_facet::<ByClaim, Hold>(&state.queue, (*title, *member, Ready::Notified)),
                Ok(Event::Collected {
                    title: *title,
                    member: *member,
                }),
                Err(Fault::NoClaim),
            ),
            Command::PassDay => Ok(Event::DayPassed),
        }
    }

    fn apply(state: State, event: &Event) -> State {
        match event {
            Event::Placed { title, member } => State {
                queue: with(
                    &state.queue,
                    Hold {
                        title: *title,
                        member: *member,
                        ready: Ready::Waiting,
                        left: COLLECT_DAYS,
                    },
                ),
            },
            Event::Notified { title, member } => State {
                queue: with(
                    &without(&state.queue, (*title, *member)),
                    Hold {
                        title: *title,
                        member: *member,
                        ready: Ready::Notified,
                        left: COLLECT_DAYS,
                    },
                ),
            },
            Event::Collected { title, member } => State {
                queue: without(&state.queue, (*title, *member)),
            },
            Event::DayPassed => State {
                queue: keep_above(&advance_all(&state.queue, 1), 0),
            },
        }
    }
}
