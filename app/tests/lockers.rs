use app::lockers::{
    abandoned, banked, current, fee, full, held_by, letting, out, rate, reclaimed, rented,
    salvage_of, step, steps, transferable,
};
use spec::contexts::lockers::state::State;
use spec::contexts::lockers::vocabulary::{
    Command, Contents, Fault, Moved, Size, CARGO_PENCE_PER_DAY, COMPACT_PENCE_PER_DAY,
    LOCKER_BAYS, LOCKER_GRACE_DAYS, LOCKER_TERM_DAYS, STANDARD_PENCE_PER_DAY,
};

fn rent(locker: u32, renter: u32, size: Size) -> Command {
    Command::Rent {
        locker,
        renter,
        size,
    }
}

fn days(how_many: u32) -> Vec<Command> {
    (0..how_many).map(|_| Command::PassDay).collect()
}

fn after(commands: Vec<Command>) -> State {
    steps(current(&[]), &commands).expect("the commands were all legal")
}

fn refused(commands: Vec<Command>) -> Fault {
    match steps(current(&[]), &commands) {
        Ok(_) => panic!("the last command was meant to be refused"),
        Err((_, fault)) => fault,
    }
}

fn then(mut first: Vec<Command>, rest: Vec<Command>) -> Vec<Command> {
    first.extend(rest);
    first
}

#[test]
fn a_locker_is_let_for_a_term() {
    let state = step(current(&[]), &rent(1, 10, Size::Standard)).expect("the first let is free");
    let held = letting(&state, 1).expect("locker one is let");
    assert_eq!(held.renter, 10);
    assert_eq!(held.left, LOCKER_TERM_DAYS);
    assert!(rented(&state, 1));
    assert!(held_by(&state, 1, 10));
    assert_eq!(out(&state, Size::Standard), 1);
    assert_eq!(banked(&state), 1);
}

#[test]
fn one_locker_is_let_to_one_renter_at_a_time() {
    let fault = refused(vec![rent(1, 10, Size::Standard), rent(1, 11, Size::Compact)]);
    assert_eq!(fault, Fault::AlreadyRented);
}

#[test]
fn the_size_sets_the_rate_and_the_term_fee() {
    assert_eq!(rate(&Size::Compact), COMPACT_PENCE_PER_DAY);
    assert_eq!(rate(&Size::Standard), STANDARD_PENCE_PER_DAY);
    assert_eq!(rate(&Size::Cargo), CARGO_PENCE_PER_DAY);
    assert_eq!(fee(&Size::Compact), COMPACT_PENCE_PER_DAY * LOCKER_TERM_DAYS);
    assert_eq!(
        fee(&Size::Standard),
        STANDARD_PENCE_PER_DAY * LOCKER_TERM_DAYS
    );
    assert_eq!(fee(&Size::Cargo), CARGO_PENCE_PER_DAY * LOCKER_TERM_DAYS);
    assert!(fee(&Size::Compact) < fee(&Size::Standard));
    assert!(fee(&Size::Cargo) > fee(&Size::Standard));
}

#[test]
fn the_term_runs_down_a_day_at_a_time() {
    let state = after(then(vec![rent(1, 10, Size::Standard)], days(3)));
    let held = letting(&state, 1).expect("locker one is still let");
    assert_eq!(held.left, LOCKER_TERM_DAYS - 3);
}

#[test]
fn a_transfer_carries_the_remaining_term_to_the_new_renter() {
    let spent = 30;
    let state = after(then(
        then(vec![rent(1, 10, Size::Cargo)], days(spent)),
        vec![Command::Transfer {
            locker: 1,
            from: 10,
            to: 11,
        }],
    ));
    let held = letting(&state, 1).expect("locker one is still let");
    assert_eq!(held.renter, 11);
    assert_eq!(held.left, LOCKER_TERM_DAYS - spent);
    assert!(held.moved == Moved::Passed);
    assert!(held_by(&state, 1, 11));
    assert!(!held_by(&state, 1, 10));
}

#[test]
fn a_transfer_keeps_the_size_the_locker_was_let_at() {
    let state = after(vec![
        rent(1, 10, Size::Cargo),
        Command::Transfer {
            locker: 1,
            from: 10,
            to: 11,
        },
    ]);
    assert_eq!(out(&state, Size::Cargo), 1);
    assert_eq!(out(&state, Size::Standard), 0);
}

fn passed_on() -> Vec<Command> {
    vec![
        rent(1, 10, Size::Standard),
        Command::Transfer {
            locker: 1,
            from: 10,
            to: 11,
        },
    ]
}

#[test]
fn a_term_may_be_transferred_once_and_no_more() {
    let state = after(passed_on());
    assert!(!transferable(&state, 1, 11));
    let fault = refused(then(
        passed_on(),
        vec![Command::Transfer {
            locker: 1,
            from: 11,
            to: 12,
        }],
    ));
    assert_eq!(fault, Fault::AlreadyTransferred);
}

#[test]
fn only_the_renter_may_transfer_the_term() {
    let fault = refused(vec![
        rent(1, 10, Size::Standard),
        Command::Transfer {
            locker: 1,
            from: 99,
            to: 11,
        },
    ]);
    assert_eq!(fault, Fault::NotTheRenter);
}

#[test]
fn a_locker_nobody_rents_cannot_be_transferred() {
    let fault = refused(vec![Command::Transfer {
        locker: 1,
        from: 10,
        to: 11,
    }]);
    assert_eq!(fault, Fault::NotRented);
}

#[test]
fn a_locker_is_not_abandoned_until_the_grace_period_runs_out() {
    let nearly = after(then(
        vec![rent(1, 10, Size::Standard)],
        days(LOCKER_TERM_DAYS + LOCKER_GRACE_DAYS - 1),
    ));
    assert!(!abandoned(&nearly, 1));
    let over = after(then(
        vec![rent(1, 10, Size::Standard)],
        days(LOCKER_TERM_DAYS + LOCKER_GRACE_DAYS),
    ));
    assert!(abandoned(&over, 1));
}

#[test]
fn a_locker_still_within_its_grace_period_is_not_reclaimed() {
    let fault = refused(then(
        then(vec![rent(1, 10, Size::Standard)], days(LOCKER_TERM_DAYS)),
        vec![Command::Reclaim {
            locker: 1,
            contents: Contents::Cycle,
        }],
    ));
    assert_eq!(fault, Fault::WithinTerm);
}

#[test]
fn an_abandoned_locker_is_reclaimed_and_its_contents_logged() {
    let state = after(then(
        then(
            vec![rent(1, 10, Size::Standard)],
            days(LOCKER_TERM_DAYS + LOCKER_GRACE_DAYS),
        ),
        vec![Command::Reclaim {
            locker: 1,
            contents: Contents::Cycle,
        }],
    ));
    assert!(!rented(&state, 1), "the locker is free again");
    assert_eq!(banked(&state), 0);
    assert_eq!(reclaimed(&state, Contents::Cycle), 1);
    assert_eq!(reclaimed(&state, Contents::Empty), 0);
    let logged = salvage_of(&state, 1).expect("the reclaim was logged");
    assert_eq!(logged.renter, 10);
    assert!(logged.contents == Contents::Cycle);
}

#[test]
fn a_reclaimed_locker_can_be_let_again() {
    let state = after(then(
        then(
            then(
                vec![rent(1, 10, Size::Standard)],
                days(LOCKER_TERM_DAYS + LOCKER_GRACE_DAYS),
            ),
            vec![Command::Reclaim {
                locker: 1,
                contents: Contents::Empty,
            }],
        ),
        vec![rent(1, 12, Size::Compact)],
    ));
    let held = letting(&state, 1).expect("locker one is let again");
    assert_eq!(held.renter, 12);
    assert_eq!(held.left, LOCKER_TERM_DAYS);
}

#[test]
fn a_locker_nobody_rents_cannot_be_reclaimed() {
    let fault = refused(vec![Command::Reclaim {
        locker: 1,
        contents: Contents::Empty,
    }]);
    assert_eq!(fault, Fault::NotRented);
}

#[test]
fn a_transferred_term_still_runs_out_and_the_locker_is_reclaimed_from_the_new_renter() {
    let spent = 30;
    let state = after(then(
        then(
            then(vec![rent(1, 10, Size::Standard)], days(spent)),
            vec![Command::Transfer {
                locker: 1,
                from: 10,
                to: 11,
            }],
        ),
        days(LOCKER_TERM_DAYS - spent + LOCKER_GRACE_DAYS),
    ));
    assert!(abandoned(&state, 1));
    let held = letting(&state, 1).expect("nobody has reclaimed it yet");
    assert_eq!(held.renter, 11);
}

#[test]
fn a_refused_batch_names_the_position_of_the_command_that_failed() {
    let outcome = steps(
        current(&[]),
        &[
            rent(1, 10, Size::Standard),
            rent(2, 11, Size::Compact),
            rent(1, 12, Size::Cargo),
        ],
    );
    assert_eq!(outcome.err(), Some((2, Fault::AlreadyRented)));
}

#[test]
fn an_empty_forecourt_is_not_full() {
    assert!(!full(&current(&[])));
}

#[test]
fn a_full_forecourt_refuses_the_next_rental_with_its_index() {
    let fill: Vec<Command> = (0..LOCKER_BAYS as u32)
        .map(|n| rent(n, n, Size::Standard))
        .collect();
    let state = after(fill);
    assert!(full(&state));
    assert_eq!(
        steps(state, &[rent(LOCKER_BAYS as u32, 9, Size::Standard)]).err(),
        Some((0, Fault::Full))
    );
}

#[test]
fn a_rented_locker_is_refused_before_the_forecourt_is_judged_full() {
    let fill: Vec<Command> = (0..LOCKER_BAYS as u32)
        .map(|n| rent(n, n, Size::Standard))
        .collect();
    let state = after(fill);
    assert_eq!(steps(state, &[rent(0, 9, Size::Standard)]).err(), Some((0, Fault::AlreadyRented)));
}
