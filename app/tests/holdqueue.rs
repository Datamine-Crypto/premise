use app::holdqueue::{allowance, at_stage, claimed, current, depth, queued, queuing, step, steps};
use spec::contexts::holdqueue::state::State;
use spec::contexts::holdqueue::vocabulary::{
    Command, Fault, Ready, Standing, COLLECT_DAYS, MEMBER_HOLDS, STAFF_HOLDS,
};

fn place(title: u32, member: u32, standing: Standing) -> Command {
    Command::Place {
        title,
        member,
        standing,
    }
}

fn title(n: usize) -> u32 {
    u32::try_from(n).expect("a hold count fits a title number")
}

fn empty() -> State {
    current(&[])
}

fn days(state: State, how_many: u32) -> State {
    let ticks: Vec<Command> = (0..how_many).map(|_| Command::PassDay).collect();
    steps(state, &ticks).expect("a day always passes")
}

#[test]
fn a_hold_joins_the_queue_for_a_title() {
    let state = step(empty(), &place(1, 10, Standing::Member)).expect("the first hold is free");
    assert!(queuing(&state, 1, 10));
    assert_eq!(queued(&state, 10), 1);
}

#[test]
fn holds_are_served_in_the_order_they_were_placed() {
    let state = steps(
        empty(),
        &[
            place(1, 10, Standing::Member),
            place(1, 11, Standing::Member),
        ],
    )
    .expect("two members may queue for one title");
    let state = step(state, &Command::Available { title: 1 }).expect("the title came back");
    assert!(claimed(&state, 1, 10), "the earlier hold was not served first");
    assert!(!claimed(&state, 1, 11));
}

#[test]
fn a_notified_hold_expires_when_it_is_not_collected_in_time() {
    let state = step(empty(), &place(1, 10, Standing::Member)).expect("a hold is placed");
    let state = step(state, &Command::Available { title: 1 }).expect("the title came back");
    let nearly = days(state.clone(), COLLECT_DAYS - 1);
    assert!(claimed(&nearly, 1, 10), "the hold expired a day early");
    let over = days(state, COLLECT_DAYS);
    assert!(!queuing(&over, 1, 10), "the hold outlived its collection window");
}

#[test]
fn a_waiting_hold_has_no_clock_running() {
    let state = step(empty(), &place(1, 10, Standing::Member)).expect("a hold is placed");
    let later = days(state, COLLECT_DAYS + COLLECT_DAYS);
    assert!(
        queuing(&later, 1, 10),
        "a hold expired before the title was ever available"
    );
}

#[test]
fn an_expired_hold_hands_the_title_to_the_next_in_line() {
    let state = steps(
        empty(),
        &[
            place(1, 10, Standing::Member),
            place(1, 11, Standing::Member),
        ],
    )
    .expect("two members may queue");
    let state = step(state, &Command::Available { title: 1 }).expect("the title came back");
    let state = days(state, COLLECT_DAYS);
    assert!(!queuing(&state, 1, 10));
    let state = step(state, &Command::Available { title: 1 }).expect("the title is free again");
    assert!(claimed(&state, 1, 11), "the second in line was skipped");
}

#[test]
fn a_title_already_offered_is_not_offered_twice() {
    let state = steps(
        empty(),
        &[
            place(1, 10, Standing::Member),
            place(1, 11, Standing::Member),
        ],
    )
    .expect("two members may queue");
    let state = step(state, &Command::Available { title: 1 }).expect("the title came back");
    assert!(at_stage(&state, 1, Ready::Notified));
    assert_eq!(
        step(state, &Command::Available { title: 1 }).err(),
        Some(Fault::AlreadyOffered)
    );
}

#[test]
fn an_empty_queue_has_nobody_to_notify() {
    assert_eq!(
        step(empty(), &Command::Available { title: 1 }).err(),
        Some(Fault::NoQueue)
    );
}

#[test]
fn a_member_may_not_pass_the_member_allowance() {
    let wanted: Vec<Command> = (0..MEMBER_HOLDS)
        .map(|n| place(title(n), 10, Standing::Member))
        .collect();
    let state = steps(empty(), &wanted).expect("the allowance covers these");
    assert_eq!(queued(&state, 10), MEMBER_HOLDS);
    assert_eq!(
        step(state, &place(title(MEMBER_HOLDS), 10, Standing::Member)).err(),
        Some(Fault::AtAllowance)
    );
}

#[test]
fn staff_may_hold_more_titles_than_a_member() {
    assert!(allowance(&Standing::Staff) > allowance(&Standing::Member));
    let wanted: Vec<Command> = (0..MEMBER_HOLDS + 1)
        .map(|n| place(title(n), 20, Standing::Staff))
        .collect();
    let state = steps(empty(), &wanted).expect("staff reach past the member allowance");
    assert_eq!(queued(&state, 20), MEMBER_HOLDS + 1);
    let over: Vec<Command> = (MEMBER_HOLDS + 1..STAFF_HOLDS + 1)
        .map(|n| place(title(n), 20, Standing::Staff))
        .collect();
    assert_eq!(
        steps(state, &over).err(),
        Some((STAFF_HOLDS - MEMBER_HOLDS - 1, Fault::AtAllowance))
    );
}

#[test]
fn one_member_cannot_queue_for_the_same_title_twice() {
    let state = step(empty(), &place(1, 10, Standing::Member)).expect("the first hold is free");
    assert_eq!(
        step(state, &place(1, 10, Standing::Member)).err(),
        Some(Fault::AlreadyQueued)
    );
}

#[test]
fn a_title_cannot_be_collected_without_an_offer() {
    let state = step(empty(), &place(1, 10, Standing::Member)).expect("a hold is placed");
    assert_eq!(
        step(state, &Command::Collect { title: 1, member: 10 }).err(),
        Some(Fault::NoClaim)
    );
}

#[test]
fn collecting_clears_the_hold_and_frees_the_allowance() {
    let state = step(empty(), &place(1, 10, Standing::Member)).expect("a hold is placed");
    let state = step(state, &Command::Available { title: 1 }).expect("the title came back");
    let state = step(state, &Command::Collect { title: 1, member: 10 }).expect("the offer stood");
    assert_eq!(queued(&state, 10), 0);
    assert_eq!(depth(&state), 0);
}

#[test]
fn a_member_who_is_not_next_cannot_collect() {
    let state = steps(
        empty(),
        &[
            place(1, 10, Standing::Member),
            place(1, 11, Standing::Member),
        ],
    )
    .expect("two members may queue");
    let state = step(state, &Command::Available { title: 1 }).expect("the title came back");
    assert_eq!(
        step(state, &Command::Collect { title: 1, member: 11 }).err(),
        Some(Fault::NoClaim)
    );
}
