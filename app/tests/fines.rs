use app::fines::{
    borrowed, ceiling, current, current_loans, enrolled, grade_text, lent, loan_of, member_of, out,
    overdue, step, steps,
};
use spec::contexts::fines::state::State;
use spec::contexts::fines::vocabulary::{
    Command, Fault, Grade, ADULT_FINE_CAP_PENCE, BORROW_BAR_PENCE, CHARGES_DAY,
    LEGACY_PENCE_PER_DAY, LOAN_DAYS, PENCE_PER_DAY,
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

fn enrol(state: State, member: u32, grade: Grade) -> State {
    one(state, &Command::Enrol { member, grade }).expect("a new member enrols")
}

fn borrow(state: State, loan: u32, member: u32) -> Result<State, Fault> {
    one(state, &Command::Borrow { loan, member })
}

fn give_back(state: State, loan: u32) -> Result<State, Fault> {
    one(state, &Command::Return { loan })
}

fn owed(state: &State, member: u32) -> u32 {
    member_of(state, member).expect("the member enrolled").owed
}

fn due(state: &State, loan: u32) -> u32 {
    loan_of(state, loan).expect("the loan is out").due
}

fn adult_owing(fine_days: u32) -> State {
    let state = enrol(empty(), 1, Grade::Adult);
    let state = borrow(state, 1, 1).expect("a new member owes nothing");
    let state = days(state, LOAN_DAYS + fine_days);
    give_back(state, 1).expect("the book comes back")
}

#[test]
fn a_loan_falls_due_a_fixed_term_after_it_is_taken_out() {
    let state = enrol(empty(), 1, Grade::Adult);
    let state = borrow(state, 1, 1).expect("a new member may borrow");
    assert!(out(&state, 1));
    assert_eq!(due(&state, 1), LOAN_DAYS);
    assert_eq!(borrowed(&state, 1), 1);
    assert_eq!(lent(&state), 1);
}

#[test]
fn a_book_brought_back_on_its_due_day_is_not_overdue() {
    let state = enrol(empty(), 1, Grade::Adult);
    let state = borrow(state, 1, 1).expect("a new member may borrow");
    let state = days(state, LOAN_DAYS);
    let state = give_back(state, 1).expect("the book comes back");
    assert_eq!(owed(&state, 1), 0);
    assert!(!out(&state, 1));
    assert_eq!(lent(&state), 0);
}

#[test]
fn an_overdue_loan_is_charged_for_every_day_past_its_due_day() {
    let state = adult_owing(3);
    assert_eq!(owed(&state, 1), 3 * LEGACY_PENCE_PER_DAY);
}

#[test]
fn the_charge_on_one_loan_stops_at_the_cap() {
    let late = 100;
    let state = adult_owing(late);
    assert!(late * LEGACY_PENCE_PER_DAY > ADULT_FINE_CAP_PENCE);
    assert_eq!(owed(&state, 1), ADULT_FINE_CAP_PENCE);
}

#[test]
fn a_junior_member_is_charged_nothing_however_late() {
    let state = enrol(empty(), 2, Grade::Junior);
    let state = borrow(state, 7, 2).expect("a junior may borrow");
    let state = days(state, LOAN_DAYS + 100);
    let state = give_back(state, 7).expect("the book comes back");
    assert_eq!(owed(&state, 2), 0);
    assert_eq!(ceiling(&Grade::Junior), 0);
}

#[test]
fn a_loan_that_fell_due_before_the_review_stays_on_the_old_rate() {
    let state = adult_owing(3);
    assert_eq!(owed(&state, 1), 3 * LEGACY_PENCE_PER_DAY);
}

#[test]
fn a_loan_that_fell_due_after_the_review_pays_the_new_rate() {
    let state = enrol(empty(), 1, Grade::Adult);
    let state = days(state, CHARGES_DAY);
    let state = borrow(state, 1, 1).expect("a new member may borrow");
    assert!(due(&state, 1) >= CHARGES_DAY);
    let state = days(state, LOAN_DAYS + 3);
    let state = give_back(state, 1).expect("the book comes back");
    assert_eq!(owed(&state, 1), 3 * PENCE_PER_DAY);
}

#[test]
fn the_two_rates_are_both_live_and_charge_differently_for_the_same_lateness() {
    let old = adult_owing(3);
    let new = enrol(empty(), 1, Grade::Adult);
    let new = days(new, CHARGES_DAY);
    let new = borrow(new, 1, 1).expect("a new member may borrow");
    let new = days(new, LOAN_DAYS + 3);
    let new = give_back(new, 1).expect("the book comes back");
    assert!(owed(&new, 1) > owed(&old, 1));
}

#[test]
fn a_member_owing_more_than_the_bar_cannot_borrow() {
    let state = adult_owing(100);
    assert!(owed(&state, 1) > BORROW_BAR_PENCE);
    assert_eq!(refused(borrow(state, 2, 1)), Fault::Barred);
}

#[test]
fn a_member_owing_exactly_the_bar_may_still_borrow() {
    let state = adult_owing(100);
    let settle = ADULT_FINE_CAP_PENCE - BORROW_BAR_PENCE;
    let state = one(
        state,
        &Command::Pay {
            member: 1,
            amount: settle,
        },
    )
    .expect("a member with a balance may pay");
    assert_eq!(owed(&state, 1), BORROW_BAR_PENCE);
    let state = borrow(state, 2, 1).expect("the bar is what a member may owe and still borrow");
    assert!(out(&state, 2));
}

#[test]
fn paying_down_below_the_bar_lets_a_member_borrow_again() {
    let state = adult_owing(100);
    let state = one(
        state,
        &Command::Pay {
            member: 1,
            amount: ADULT_FINE_CAP_PENCE,
        },
    )
    .expect("a member with a balance may pay");
    assert_eq!(owed(&state, 1), 0);
    let state = borrow(state, 2, 1).expect("a member who owes nothing may borrow");
    assert!(out(&state, 2));
}

#[test]
fn a_payment_larger_than_the_balance_settles_the_balance_and_no_more() {
    let state = adult_owing(3);
    let state = one(
        state,
        &Command::Pay {
            member: 1,
            amount: ADULT_FINE_CAP_PENCE,
        },
    )
    .expect("a member with a balance may pay");
    assert_eq!(owed(&state, 1), 0);
    assert_eq!(
        refused(one(
            state,
            &Command::Pay {
                member: 1,
                amount: 1
            }
        )),
        Fault::NothingOwed
    );
}

#[test]
fn charges_from_several_late_loans_add_up_against_one_member() {
    let state = enrol(empty(), 1, Grade::Adult);
    let state = many(
        state,
        &[
            Command::Borrow {
                loan: 1,
                member: 1,
            },
            Command::Borrow {
                loan: 2,
                member: 1,
            },
        ],
    )
    .expect("a new member may borrow twice");
    assert_eq!(borrowed(&state, 1), 2);
    let state = days(state, LOAN_DAYS + 3);
    let state = give_back(state, 1).expect("the first book comes back");
    let state = give_back(state, 2).expect("the second book comes back");
    assert_eq!(owed(&state, 1), 2 * (3 * LEGACY_PENCE_PER_DAY));
    assert_eq!(lent(&state), 0);
}

#[test]
fn the_desk_refuses_what_it_has_no_record_of() {
    let state = enrol(empty(), 1, Grade::Adult);
    assert!(enrolled(&state, 1));
    assert!(!enrolled(&state, 2));
    assert_eq!(
        refused(one(state.clone(), &Command::Enrol { member: 1, grade: Grade::Junior })),
        Fault::AlreadyEnrolled
    );
    assert_eq!(refused(borrow(state.clone(), 1, 2)), Fault::NotEnrolled);
    assert_eq!(refused(give_back(state.clone(), 1)), Fault::NoLoan);
    assert_eq!(
        refused(one(state, &Command::Pay { member: 2, amount: 1 })),
        Fault::NotEnrolled
    );
}

#[test]
fn one_book_cannot_be_lent_twice() {
    let state = enrol(empty(), 1, Grade::Adult);
    let state = enrol(state, 2, Grade::Adult);
    let state = borrow(state, 1, 1).expect("the first member takes it out");
    assert_eq!(refused(borrow(state, 1, 2)), Fault::AlreadyOnLoan);
}

#[test]
fn a_grade_carries_its_own_text_and_its_own_ceiling() {
    assert_eq!(grade_text(&Grade::Junior), "junior");
    assert_eq!(grade_text(&Grade::Adult), "adult");
    assert_eq!(ceiling(&Grade::Adult), ADULT_FINE_CAP_PENCE);
}

#[test]
fn a_loan_counts_as_overdue_only_once_the_day_after_its_due_day_starts() {
    let state = enrol(empty(), 1, Grade::Adult);
    let state = borrow(state, 1, 1).expect("a new member may borrow");
    assert_eq!(overdue(&state), 0);
    assert_eq!(current_loans(&state), 1);
    let state = days(state, LOAN_DAYS);
    assert_eq!(overdue(&state), 0);
    assert_eq!(current_loans(&state), 1);
    let state = days(state, 1);
    assert_eq!(overdue(&state), 1);
    assert_eq!(current_loans(&state), 0);
}

#[test]
fn the_two_counts_partition_the_loans_whatever_the_day() {
    let state = enrol(empty(), 1, Grade::Adult);
    let state = borrow(state, 1, 1).expect("a new member may borrow");
    let state = days(state, LOAN_DAYS);
    let state = borrow(state, 2, 1).expect("a member under the bar may borrow again");
    let state = days(state, LOAN_DAYS);
    assert_eq!(overdue(&state), 1);
    assert_eq!(current_loans(&state), 1);
    assert_eq!(overdue(&state) + current_loans(&state), lent(&state));
}
