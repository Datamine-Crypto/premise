use app::seasonticket::{
    allowance, cover_text, current, issued, let_out, running, sold, step, steps, ticket_of,
    tier_text,
};
use spec::contexts::seasonticket::state::State;
use spec::contexts::seasonticket::vocabulary::{
    Command, Cover, Fault, Tier, LEGACY_REASSIGN_PENCE, REASSIGN_PENCE, RESIDENT_SUSPEND_DAYS,
    TARIFF_DAY, TERM_DAYS, VISITOR_SUSPEND_DAYS,
};

fn empty() -> State {
    current(&[])
}

fn one(state: State, command: &Command) -> Result<State, Fault> {
    step(state, command)
}

fn many(state: State, commands: &[Command]) -> Result<State, (usize, Fault)> {
    steps(state, commands)
}

fn refused(outcome: Result<State, Fault>) -> Fault {
    outcome.err().expect("the command was refused")
}

fn days(state: State, how_many: u32) -> State {
    let ticks: Vec<Command> = (0..how_many).map(|_| Command::PassDay).collect();
    many(state, &ticks).expect("a day always passes")
}

fn buy(ticket: u32, bay: u32, tier: Tier) -> Command {
    Command::Buy { ticket, bay, tier }
}

fn expiry(state: &State, ticket: u32) -> u32 {
    ticket_of(state, ticket).expect("the ticket was issued").expires
}

fn used(state: &State, ticket: u32) -> u32 {
    ticket_of(state, ticket).expect("the ticket was issued").used
}

fn owed(state: &State, ticket: u32) -> u32 {
    ticket_of(state, ticket).expect("the ticket was issued").owed
}

fn cover(state: &State, ticket: u32) -> Cover {
    ticket_of(state, ticket).expect("the ticket was issued").cover
}

fn bay(state: &State, ticket: u32) -> u32 {
    ticket_of(state, ticket).expect("the ticket was issued").bay
}

#[test]
fn a_season_covers_one_bay_for_the_term() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    assert!(issued(&state, 1));
    assert!(let_out(&state, 40));
    assert_eq!(expiry(&state, 1), TERM_DAYS);
    assert_eq!(running(&state), 1);
}

#[test]
fn a_bay_carries_one_season_at_a_time() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    assert_eq!(refused(one(state, &buy(2, 40, Tier::Visitor))), Fault::BayTaken);
}

#[test]
fn a_ticket_number_is_issued_once() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    assert_eq!(
        refused(one(state, &buy(1, 41, Tier::Resident))),
        Fault::AlreadyIssued
    );
}

#[test]
fn suspended_days_push_the_expiry_out_by_the_days_away() {
    let state = one(empty(), &buy(1, 40, Tier::Visitor)).expect("the first season is free");
    let state = days(state, 5);
    let state = one(state, &Command::Suspend { ticket: 1 }).expect("an active season stands down");
    assert_eq!(cover(&state, 1), Cover::Suspended);
    let state = days(state, 3);
    let state = one(state, &Command::Resume { ticket: 1 }).expect("a stood down season resumes");
    assert_eq!(cover(&state, 1), Cover::Active);
    assert_eq!(expiry(&state, 1), TERM_DAYS + 3);
    assert_eq!(used(&state, 1), 3);
}

#[test]
fn days_away_beyond_the_tier_allowance_are_not_granted() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = one(state, &Command::Suspend { ticket: 1 }).expect("an active season stands down");
    let state = days(state, RESIDENT_SUSPEND_DAYS + 20);
    let state = one(state, &Command::Resume { ticket: 1 }).expect("a stood down season resumes");
    assert_eq!(expiry(&state, 1), TERM_DAYS + RESIDENT_SUSPEND_DAYS);
    assert_eq!(used(&state, 1), RESIDENT_SUSPEND_DAYS);
}

#[test]
fn a_resident_may_stand_down_longer_than_a_visitor() {
    let long = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let long = one(long, &Command::Suspend { ticket: 1 }).expect("an active season stands down");
    let long = days(long, TERM_DAYS);
    let long = one(long, &Command::Resume { ticket: 1 }).expect("a stood down season resumes");

    let short = one(empty(), &buy(2, 41, Tier::Visitor)).expect("the first season is free");
    let short = one(short, &Command::Suspend { ticket: 2 }).expect("an active season stands down");
    let short = days(short, TERM_DAYS);
    let short = one(short, &Command::Resume { ticket: 2 }).expect("a stood down season resumes");

    assert_eq!(used(&long, 1), RESIDENT_SUSPEND_DAYS);
    assert_eq!(used(&short, 2), VISITOR_SUSPEND_DAYS);
    assert!(used(&long, 1) > used(&short, 2));
}

#[test]
fn the_allowance_is_spent_across_several_absences_not_reset_by_each() {
    let state = one(empty(), &buy(1, 40, Tier::Visitor)).expect("the first season is free");
    let state = one(state, &Command::Suspend { ticket: 1 }).expect("an active season stands down");
    let state = days(state, 3);
    let state = one(state, &Command::Resume { ticket: 1 }).expect("a stood down season resumes");
    let state = one(state, &Command::Suspend { ticket: 1 }).expect("some allowance is left");
    let state = days(state, 10);
    let state = one(state, &Command::Resume { ticket: 1 }).expect("a stood down season resumes");

    assert_eq!(used(&state, 1), VISITOR_SUSPEND_DAYS);
    assert_eq!(expiry(&state, 1), TERM_DAYS + VISITOR_SUSPEND_DAYS);
    assert_eq!(
        refused(one(state, &Command::Suspend { ticket: 1 })),
        Fault::AllowanceSpent
    );
}

#[test]
fn a_season_already_stood_down_cannot_stand_down_again() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = one(state, &Command::Suspend { ticket: 1 }).expect("an active season stands down");
    assert_eq!(
        refused(one(state, &Command::Suspend { ticket: 1 })),
        Fault::AlreadySuspended
    );
}

#[test]
fn an_active_season_cannot_resume() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    assert_eq!(
        refused(one(state, &Command::Resume { ticket: 1 })),
        Fault::NotSuspended
    );
}

#[test]
fn a_command_naming_no_season_faults() {
    assert_eq!(
        refused(one(empty(), &Command::Suspend { ticket: 9 })),
        Fault::NoTicket
    );
    assert_eq!(
        refused(one(empty(), &Command::Reassign { ticket: 9, bay: 40 })),
        Fault::NoTicket
    );
}

#[test]
fn a_lapsed_season_can_neither_stand_down_nor_move() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = days(state, TERM_DAYS);
    assert_eq!(running(&state), 0);
    assert_eq!(
        refused(one(state.clone(), &Command::Suspend { ticket: 1 })),
        Fault::Lapsed
    );
    assert_eq!(
        refused(one(state, &Command::Reassign { ticket: 1, bay: 41 })),
        Fault::Lapsed
    );
}

#[test]
fn a_move_lands_the_holder_on_the_new_bay_and_frees_the_old() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = one(state, &Command::Reassign { ticket: 1, bay: 41 })
        .expect("the old bay is out of service");
    assert_eq!(bay(&state, 1), 41);
    assert!(let_out(&state, 41));
    assert!(!let_out(&state, 40));
}

#[test]
fn a_move_to_the_same_bay_or_to_a_taken_one_is_refused() {
    let state = many(
        empty(),
        &[buy(1, 40, Tier::Resident), buy(2, 41, Tier::Visitor)],
    )
    .expect("two bays take two seasons");
    assert_eq!(
        refused(one(state.clone(), &Command::Reassign { ticket: 1, bay: 40 })),
        Fault::SameBay
    );
    assert_eq!(
        refused(one(state, &Command::Reassign { ticket: 1, bay: 41 })),
        Fault::BayTaken
    );
}

#[test]
fn a_season_bought_before_the_tariff_day_still_owes_the_old_fee_after_it() {
    let state = days(empty(), TARIFF_DAY - 30);
    let state = one(state, &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = days(state, 40);
    let state = one(state, &Command::Reassign { ticket: 1, bay: 41 })
        .expect("the old bay is out of service");
    assert_eq!(owed(&state, 1), LEGACY_REASSIGN_PENCE);
}

#[test]
fn a_season_bought_on_or_after_the_tariff_day_owes_the_new_fee() {
    let state = days(empty(), TARIFF_DAY);
    let state = one(state, &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = one(state, &Command::Reassign { ticket: 1, bay: 41 })
        .expect("the old bay is out of service");
    assert_eq!(owed(&state, 1), REASSIGN_PENCE);
}

#[test]
fn two_moves_owe_two_fees() {
    let state = days(empty(), TARIFF_DAY);
    let state = one(state, &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = many(
        state,
        &[
            Command::Reassign { ticket: 1, bay: 41 },
            Command::Reassign { ticket: 1, bay: 42 },
        ],
    )
    .expect("a bay can go out of service twice");
    assert_eq!(owed(&state, 1), REASSIGN_PENCE + REASSIGN_PENCE);
    assert_eq!(bay(&state, 1), 42);
}

#[test]
fn surrendering_a_season_frees_the_bay() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = one(state, &Command::Surrender { ticket: 1 }).expect("a holder may give up a bay");
    assert!(!issued(&state, 1));
    assert!(!let_out(&state, 40));
    let state = one(state, &buy(2, 40, Tier::Visitor)).expect("the freed bay lets again");
    assert!(let_out(&state, 40));
}

#[test]
fn a_tier_carries_its_own_text() {
    assert_eq!(tier_text(&Tier::Resident), "resident");
    assert_eq!(tier_text(&Tier::Visitor), "visitor");
    assert_eq!(cover_text(&Cover::Active), "active");
    assert_eq!(cover_text(&Cover::Suspended), "suspended");
}

#[test]
fn a_tier_carries_the_days_it_may_stand_down_for() {
    assert_eq!(allowance(&Tier::Resident), RESIDENT_SUSPEND_DAYS);
    assert_eq!(allowance(&Tier::Visitor), VISITOR_SUSPEND_DAYS);
}

#[test]
fn a_lapsed_season_is_still_sold_until_it_is_surrendered() {
    let state = one(empty(), &buy(1, 40, Tier::Resident)).expect("the first season is free");
    let state = days(state, TERM_DAYS);
    assert_eq!(sold(&state), 1);
    assert_eq!(running(&state), 0);
    let state = one(state, &Command::Surrender { ticket: 1 }).expect("a holder may give up a bay");
    assert_eq!(sold(&state), 0);
}
