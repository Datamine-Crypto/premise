use patterns::{
    across, at, away, between, bitmap, doubled, firsts, flipped, folded, half, lanes_here,
    nearest, numbered, parted, rows_flipped, seconds, shot, without_alpha,
    refuse_when_lazily, Guard,
    all, any, clamp_lower, clamp_range, clamp_upper, count, count_below, count_from, digits, div,
    either, first, fold, handle, handle_all, is_above, is_at_least, is_at_most, is_below, is_same,
    joined, keep, label, last, map, mul, named, raise_by, reduce_by, rem, replay, replay_from,
    total, value_of, Bounded, Context, HasMax, Named, Valued,
};

struct Slot;

impl Bounded for Slot {
    type Of = u32;
    const LO: u32 = 0;
    const HI: u32 = 10;
}

#[test]
fn max_is_derived_from_bounds() {
    assert_eq!(<Slot as HasMax>::MAX, 10);
    assert_eq!(<Slot as Bounded>::HI, 10);
}

#[test]
fn clamping_holds_the_range() {
    assert_eq!(clamp_range(99, 0, 10), 10);
    assert_eq!(clamp_range(5, 0, 10), 5);
    assert_eq!(clamp_range(0, 3, 10), 3);
}

#[test]
fn a_ceiling_alone_leaves_the_low_side_open() {
    assert_eq!(clamp_upper(9, 5), 5);
    assert_eq!(clamp_upper(3, 5), 3);
    assert_eq!(clamp_upper(0, 5), 0);
}

#[test]
fn a_floor_alone_leaves_the_high_side_open() {
    assert_eq!(clamp_lower(2, 5), 5);
    assert_eq!(clamp_lower(7, 5), 7);
    assert_eq!(clamp_lower(99, 5), 99);
}

#[test]
fn saturating_moves_stop_at_the_edge() {
    assert_eq!(reduce_by(3u32, 9u32), 0);
    assert_eq!(raise_by(u32::MAX, 5u32), u32::MAX);
}

#[test]
fn scaling_stops_at_the_ceiling_instead_of_wrapping() {
    assert_eq!(mul(6u32, 7u32), 42);
    assert_eq!(mul(u32::MAX, 2u32), u32::MAX);
    assert_eq!(mul(0u32, u32::MAX), 0);
}

#[test]
fn division_by_zero_is_a_value_not_a_panic() {
    assert_eq!(div(9u32, 2u32), Some(4));
    assert_eq!(div(9u32, 0u32), None);
    assert_eq!(div(0u32, 3u32), Some(0));
}

#[test]
fn a_remainder_by_zero_is_a_value_not_a_panic() {
    assert_eq!(rem(9u32, 4u32), Some(1));
    assert_eq!(rem(8u32, 4u32), Some(0));
    assert_eq!(rem(9u32, 0u32), None);
}

#[test]
fn selection_reads_as_data() {
    assert_eq!(either(true, 1, 2), 1);
    assert_eq!(either(false, 1, 2), 2);
    assert!(is_at_most(3, 3));
}

#[test]
fn the_strict_tests_reject_their_own_boundary() {
    assert!(is_above(4, 3));
    assert!(!is_above(3, 3));
    assert!(!is_above(2, 3));
    assert!(is_below(2, 3));
    assert!(!is_below(3, 3));
    assert!(!is_below(4, 3));
}

#[test]
fn the_inclusive_test_accepts_its_boundary() {
    assert!(is_at_least(3, 3));
    assert!(is_at_least(4, 3));
    assert!(!is_at_least(2, 3));
}

#[test]
fn sameness_is_equality_and_nothing_looser() {
    assert!(is_same(3, 3));
    assert!(!is_same(3, 4));
    assert!(is_same("a", "a"));
}

fn double(v: &u32) -> u32 {
    v * 2
}

fn big(v: &u32) -> bool {
    *v > 2
}

fn never(_: &u32) -> bool {
    false
}

fn running(acc: u32, v: &u32) -> u32 {
    acc + v
}

#[test]
fn sequences_take_plain_functions() {
    let items = [1u32, 2, 3, 4];
    assert_eq!(map(&items, double), vec![2, 4, 6, 8]);
    assert_eq!(keep(&items, big), vec![3, 4]);
    assert_eq!(fold(&items, 0, running), 10);
    assert_eq!(count(&items, big), 2);
    assert!(any(&items, big));
    assert!(!all(&items, big));
}

#[test]
fn first_answers_the_earliest_match_in_order() {
    let items = [1u32, 2, 3, 4];
    assert_eq!(first(&items, big), Some(3));
    assert_eq!(first(&items, never), None);
    assert_eq!(first(&[] as &[u32], big), None);
}

#[test]
fn last_answers_the_latest_match_in_order() {
    let items = [1u32, 4, 3, 2];
    assert_eq!(last(&items, big), Some(3));
    assert_eq!(last(&items, never), None);
    assert_eq!(last(&[] as &[u32], big), None);
}

struct Counter;

enum Command {
    Bump,
    Overflow,
}

enum Event {
    Bumped,
}

#[derive(Debug, PartialEq)]
enum Fault {
    Refused,
}

#[derive(Debug, PartialEq)]
struct Count {
    value: u32,
}

impl Context for Counter {
    type State = Count;
    type Command = Command;
    type Event = Event;
    type Fault = Fault;

    fn initial() -> Count {
        Count { value: 0 }
    }

    fn decide(_state: &Count, command: &Command) -> Result<Event, Fault> {
        match command {
            Command::Bump => Ok(Event::Bumped),
            Command::Overflow => Err(Fault::Refused),
        }
    }

    fn apply(state: Count, event: &Event) -> Count {
        match event {
            Event::Bumped => Count {
                value: raise_by(state.value, 1),
            },
        }
    }
}

#[test]
fn state_is_rebuilt_from_events_alone() {
    let log = [Event::Bumped, Event::Bumped, Event::Bumped];
    assert_eq!(replay::<Counter>(&log), Count { value: 3 });
    assert_eq!(replay::<Counter>(&[]), Count { value: 0 });
}

#[test]
fn a_snapshot_catches_up_without_starting_from_initial() {
    let log = [Event::Bumped, Event::Bumped];
    assert_eq!(
        replay_from::<Counter>(Count { value: 5 }, &log),
        Count { value: 7 }
    );
    assert_eq!(
        replay_from::<Counter>(Count { value: 5 }, &[]),
        Count { value: 5 }
    );
}

#[test]
fn a_command_becomes_an_event_or_a_fault() {
    let start = <Counter as Context>::initial();
    assert_eq!(handle::<Counter>(start, &Command::Bump), Ok(Count { value: 1 }));
    let again = <Counter as Context>::initial();
    assert_eq!(handle::<Counter>(again, &Command::Overflow), Err(Fault::Refused));
}

#[test]
fn a_batch_stops_at_the_first_fault_and_says_where() {
    let start = <Counter as Context>::initial();
    let clean = [Command::Bump, Command::Bump];
    assert_eq!(handle_all::<Counter>(start, &clean), Ok(Count { value: 2 }));
    let broken = [
        Command::Bump,
        Command::Bump,
        Command::Overflow,
        Command::Bump,
    ];
    let again = <Counter as Context>::initial();
    assert_eq!(
        handle_all::<Counter>(again, &broken),
        Err((2, Fault::Refused))
    );
    let empty = <Counter as Context>::initial();
    assert_eq!(handle_all::<Counter>(empty, &[]), Ok(Count { value: 0 }));
}

#[test]
fn chance_is_deterministic_and_threads_its_seed() {
    use patterns::{next, pick, Seed};
    let start = Seed(42);
    let (a, moved) = next(start);
    let (b, _) = next(start);
    assert_eq!(a, b);
    assert_ne!(moved, start);

    let faces = ["red", "green", "blue"];
    let (chosen, _) = pick(start, &faces);
    assert!(chosen.is_some());
    let (nothing, same) = pick(start, &[] as &[u8]);
    assert!(nothing.is_none());
    assert_eq!(same, start);
}

#[test]
fn a_zero_bound_yields_zero_and_still_moves_the_seed() {
    use patterns::{below, next, Seed};
    for raw in [0u64, 1, 42, u64::MAX] {
        let start = Seed(raw);
        let (value, moved) = below(start, 0);
        assert_eq!(value, 0);
        assert_eq!(moved, next(start).1);
    }
}

#[test]
fn a_bounded_value_stays_under_its_bound_and_covers_the_range() {
    use patterns::{below, Seed};
    let mut seed = Seed(7);
    let mut seen = [false; 6];
    for _ in 0..2000 {
        for bound in [1u64, 6, 7, 1000, u64::MAX] {
            let (value, _) = below(seed, bound);
            assert!(value < bound, "{} is not under {}", value, bound);
        }
        let (face, moved) = below(seed, 6);
        seen[face as usize] = true;
        seed = moved;
    }
    assert!(seen.iter().all(|s| *s), "some face never came up: {:?}", seen);
}

#[derive(Clone, Debug, PartialEq)]
struct Loan {
    title: u32,
    days: u32,
}

impl patterns::Keyed for Loan {
    type Key = u32;

    fn key(&self) -> u32 {
        self.title
    }
}

impl patterns::Ranked for Loan {
    type Rank = u32;

    fn rank(&self) -> u32 {
        self.days
    }
}

struct ByDays;

impl Valued<ByDays> for Loan {
    type Value = u32;

    fn value(&self) -> u32 {
        self.days
    }
}

impl patterns::Advance for Loan {
    type By = u32;

    fn advanced(self, by: u32) -> Loan {
        Loan {
            title: self.title,
            days: patterns::raise_by(self.days, by),
        }
    }
}

#[test]
fn a_collection_can_be_searched_by_a_runtime_value() {
    use patterns::{advance_all, found, holds, is_empty, size, with, without};
    let shelf = vec![
        Loan { title: 7, days: 0 },
        Loan { title: 9, days: 2 },
    ];

    assert!(holds(&shelf, 7));
    assert!(!holds(&shelf, 8));
    assert_eq!(found(&shelf, 9).map(|l| l.days), Some(2));
    assert_eq!(size(&shelf), 2);
    assert!(!is_empty(&shelf));

    let added = with(&shelf, Loan { title: 4, days: 0 });
    assert_eq!(size(&added), 3);

    let removed = without(&added, 7);
    assert!(!holds(&removed, 7));
    assert_eq!(size(&removed), 2);

    let aged = advance_all(&shelf, 3);
    assert_eq!(aged[0].days, 3);
    assert_eq!(aged[1].days, 5);
}

fn shelf() -> Vec<Loan> {
    vec![
        Loan { title: 7, days: 1 },
        Loan { title: 9, days: 14 },
        Loan { title: 4, days: 30 },
    ]
}

#[test]
fn a_collection_can_be_compared_against_a_runtime_bound() {
    use patterns::{count_above, keep_above};
    let cutoff = 10;

    assert_eq!(count_above(&shelf(), cutoff), 2);
    assert_eq!(keep_above(&shelf(), cutoff).len(), 2);
    assert_eq!(count_above(&shelf(), 100), 0);
}

#[test]
fn counting_under_a_bound_excludes_the_bound_itself() {
    assert_eq!(count_below(&shelf(), 14), 1);
    assert_eq!(count_below(&shelf(), 15), 2);
    assert_eq!(count_below(&shelf(), 1), 0);
}

#[test]
fn counting_from_a_bound_includes_the_bound_itself() {
    assert_eq!(count_from(&shelf(), 14), 2);
    assert_eq!(count_from(&shelf(), 15), 1);
    assert_eq!(count_from(&shelf(), 31), 0);
}

#[test]
fn a_total_of_values_starts_where_the_caller_says_and_saturates() {
    assert_eq!(total::<ByDays, Loan>(&shelf(), 0), 45);
    assert_eq!(total::<ByDays, Loan>(&shelf(), 5), 50);
    assert_eq!(total::<ByDays, Loan>(&[], 3), 3);
    let heavy = vec![
        Loan { title: 1, days: u32::MAX },
        Loan { title: 2, days: 1 },
    ];
    assert_eq!(total::<ByDays, Loan>(&heavy, 0), u32::MAX);
}

#[derive(Clone, Debug, PartialEq)]
struct Ticket {
    line: u32,
    seat: u32,
    open: bool,
}

struct ByLine;

struct ByPair;

impl patterns::Facet<ByLine> for Ticket {
    type Value = u32;

    fn facet(&self) -> u32 {
        self.line
    }
}

impl patterns::Facet<ByPair> for Ticket {
    type Value = (u32, bool);

    fn facet(&self) -> (u32, bool) {
        (self.line, self.open)
    }
}

impl patterns::Ranked for Ticket {
    type Rank = u32;

    fn rank(&self) -> u32 {
        self.seat
    }
}

fn desk() -> Vec<Ticket> {
    vec![
        Ticket { line: 1, seat: 7, open: false },
        Ticket { line: 1, seat: 8, open: true },
        Ticket { line: 2, seat: 9, open: true },
    ]
}

#[test]
fn one_type_can_carry_two_facets_at_once() {
    let items = desk();
    assert_eq!(patterns::count_facet::<ByLine, Ticket>(&items, 1), 2);
    assert_eq!(patterns::count_facet::<ByPair, Ticket>(&items, (1, true)), 1);
}

#[test]
fn a_facet_finds_the_earliest_match_in_order() {
    let items = desk();
    let found = patterns::first_facet::<ByPair, Ticket>(&items, (1, true));
    assert_eq!(found.map(|t| t.seat), Some(8));
    assert!(patterns::first_facet::<ByLine, Ticket>(&items, 3).is_none());
}

#[test]
fn a_facet_answers_whether_any_element_matches() {
    let items = desk();
    assert!(patterns::any_facet::<ByPair, Ticket>(&items, (2, true)));
    assert!(!patterns::any_facet::<ByPair, Ticket>(&items, (2, false)));
}

#[test]
fn a_facet_keeps_every_match_in_order() {
    let items = desk();
    let kept = patterns::keep_facet::<ByLine, Ticket>(&items, 1);
    assert_eq!(kept.iter().map(|t| t.seat).collect::<Vec<u32>>(), vec![7, 8]);
    assert!(patterns::keep_facet::<ByLine, Ticket>(&items, 3).is_empty());
}

#[test]
fn a_facet_removes_every_match_and_keeps_the_rest() {
    let items = desk();
    let left = patterns::without_facet::<ByLine, Ticket>(&items, 1);
    assert_eq!(left.iter().map(|t| t.seat).collect::<Vec<u32>>(), vec![9]);
    let untouched = patterns::without_facet::<ByPair, Ticket>(&items, (2, false));
    assert_eq!(untouched.len(), 3);
}

#[test]
fn a_rank_can_be_read_without_a_method_call() {
    let items = desk();
    assert_eq!(patterns::rank_of(&items[0]), 7);
}

#[derive(Clone, Copy)]
enum Grade {
    Junior,
    Adult,
}

struct ByCap;

struct ByFee;

impl Valued<ByCap> for Grade {
    type Value = u32;

    fn value(&self) -> u32 {
        match self {
            Grade::Junior => 0,
            Grade::Adult => 500,
        }
    }
}

impl Valued<ByFee> for Grade {
    type Value = u32;

    fn value(&self) -> u32 {
        match self {
            Grade::Junior => 5,
            Grade::Adult => 20,
        }
    }
}

impl Named for Grade {
    fn text(&self) -> &'static str {
        match self {
            Grade::Junior => "junior",
            Grade::Adult => "adult",
        }
    }
}

#[test]
fn a_variant_can_stand_for_two_figures_under_two_markers() {
    assert_eq!(value_of::<ByCap, Grade>(&Grade::Adult), 500);
    assert_eq!(value_of::<ByCap, Grade>(&Grade::Junior), 0);
    assert_eq!(value_of::<ByFee, Grade>(&Grade::Adult), 20);
    assert_eq!(value_of::<ByFee, Grade>(&Grade::Junior), 5);
}

#[test]
fn a_named_variant_renders_its_own_text() {
    assert_eq!(named(&Grade::Junior), "junior");
    assert_eq!(named(&Grade::Adult), "adult");
}

#[test]
fn a_number_renders_as_its_digits() {
    assert_eq!(digits(42u32), "42");
    assert_eq!(digits(0u8), "0");
    assert_eq!(digits(-7i32), "-7");
}

#[test]
fn a_label_joins_a_name_to_its_value() {
    assert_eq!(label("hp", "3"), "hp=3");
    assert_eq!(label("", "x"), "=x");
}

#[test]
fn parts_are_joined_with_the_separator_between_them_only() {
    let parts = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    assert_eq!(joined(&parts, ", "), "a, b, c");
    assert_eq!(joined(&parts[..1], ", "), "a");
    assert_eq!(joined(&[], ", "), "");
}

#[test]
fn no_reason_leans_on_the_one_beside_it() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut dangling: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap().flatten() {
        let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
        for line in text.lines() {
            if line.contains("the same reason") || line.contains("as above") {
                dangling.push(line.trim().to_string());
            }
        }
    }
    assert!(
        dangling.is_empty(),
        "a reason that only parses in file order is the one artifact here whose meaning depends on where it sits: {:?}",
        dangling
    );
}

#[derive(Clone)]
struct Season {
    days: u32,
    fee: u32,
}

struct ByTerm;
struct ByCost;

impl patterns::Shift<ByTerm> for Season {
    type By = u32;
    fn shifted(self, by: u32) -> Self {
        Season {
            days: self.days + by,
            fee: self.fee,
        }
    }
}

impl patterns::Shift<ByCost> for Season {
    type By = u32;
    fn shifted(self, by: u32) -> Self {
        Season {
            days: self.days,
            fee: self.fee + by,
        }
    }
}

#[test]
fn one_type_may_carry_two_named_mutations() {
    let held = vec![Season { days: 10, fee: 5 }];
    let longer = patterns::shift_all::<ByTerm, Season>(&held, 3);
    let dearer = patterns::shift_all::<ByCost, Season>(&held, 3);
    assert_eq!((longer[0].days, longer[0].fee), (13, 5));
    assert_eq!((dearer[0].days, dearer[0].fee), (10, 8));
}

fn cheap() -> u32 {
    1
}

fn dear() -> u32 {
    2
}

fn boom() -> u32 {
    panic!("the branch that is not returned was built")
}

#[test]
fn the_lazy_choice_builds_only_the_branch_it_returns() {
    assert_eq!(patterns::either_lazily(true, cheap, boom), 1);
    assert_eq!(patterns::either_lazily(false, boom, dear), 2);
}

#[test]
fn every_public_trait_has_a_consumer() {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut declared: Vec<String> = Vec::new();
    let mut whole = String::new();
    for root in [here.join("src"), here.join("..").join("spec").join("src")] {
        let mut stack = vec![root];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
                let p = entry.path();
                if p.is_dir() {
                    stack.push(p);
                    continue;
                }
                let text = std::fs::read_to_string(&p).unwrap_or_default();
                if dir.starts_with(here.join("src")) {
                    for line in text.lines() {
                        if let Some(rest) = line.strip_prefix("pub trait ") {
                            declared.push(
                                rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                                    .next()
                                    .unwrap_or_default()
                                    .to_string(),
                            );
                        }
                    }
                }
                whole.push_str(&text);
            }
        }
    }
    assert!(declared.len() > 3, "the library lost its traits: {:?}", declared);
    let idle: Vec<&String> = declared
        .iter()
        .filter(|t| {
            let used = [format!(": {}", t), format!("<{}>", t), format!("+ {}", t)];
            let lived = format!("impl {} for", t);
            let blanket = format!("> {} for", t);
            !used.iter().any(|u| whole.contains(u))
                && !whole.contains(&lived)
                && !whole.contains(&blanket)
        })
        .collect();
    assert!(
        idle.is_empty(),
        "a trait nothing takes as a bound and nothing implements is a catalog entry that teaches a wrong turn: {:?}",
        idle
    );
}

#[test]
fn the_first_true_guard_wins_in_table_order() {
    use patterns::refuse_when;
    let checks = [(false, "a"), (true, "b"), (true, "c")];
    assert_eq!(refuse_when(&checks, 1u32), Err("b"));
    let none = [(false, "a"), (false, "b")];
    assert_eq!(refuse_when(&none, 1u32), Ok(1));
    assert_eq!(refuse_when(&[] as &[(bool, &str)], 2u32), Ok(2));
    let reversed = [(true, "c"), (true, "b")];
    assert_eq!(refuse_when(&reversed, 1u32), Err("c"));
}

#[test]
fn a_key_lookup_can_borrow_instead_of_cloning() {
    use patterns::found_ref;
    let shelf = shelf();
    let hit = found_ref(&shelf, 9).expect("title nine is on the shelf");
    assert!(std::ptr::eq(hit, &shelf[1]));
    assert!(found_ref(&shelf, 8).is_none());
}

#[test]
fn the_first_match_can_be_borrowed_in_order() {
    use patterns::first_ref;
    let items = [1u32, 3, 4];
    let hit = first_ref(&items, big).expect("three is big");
    assert!(std::ptr::eq(hit, &items[1]));
    assert!(first_ref(&items, never).is_none());
}

#[test]
fn survivors_can_be_borrowed_in_order() {
    use patterns::keep_ref;
    let items = [1u32, 3, 2, 4];
    let kept = keep_ref(&items, big);
    assert_eq!(kept, vec![&items[1], &items[3]]);
    assert!(keep_ref(&items, never).is_empty());
}

#[test]
fn the_first_facet_match_can_be_borrowed_in_order() {
    let items = desk();
    let hit = patterns::first_facet_ref::<ByPair, Ticket>(&items, (1, true))
        .expect("seat eight is open on line one");
    assert!(std::ptr::eq(hit, &items[1]));
    assert!(patterns::first_facet_ref::<ByLine, Ticket>(&items, 3).is_none());
}

fn stack() -> Vec<Loan> {
    vec![
        Loan { title: 1, days: 5 },
        Loan { title: 2, days: 9 },
        Loan { title: 3, days: 2 },
        Loan { title: 4, days: 9 },
        Loan { title: 5, days: 7 },
    ]
}

#[test]
fn the_highest_rank_wins_and_a_tie_goes_to_the_earliest() {
    assert_eq!(patterns::highest(&stack()), Some(Loan { title: 2, days: 9 }));
    assert_eq!(patterns::highest::<Loan>(&[]), None);
}

#[test]
fn the_lowest_rank_wins_and_a_tie_goes_to_the_earliest() {
    let two = vec![Loan { title: 8, days: 2 }, Loan { title: 3, days: 2 }];
    assert_eq!(patterns::lowest(&stack()), Some(Loan { title: 3, days: 2 }));
    assert_eq!(patterns::lowest(&two), Some(Loan { title: 8, days: 2 }));
    assert_eq!(patterns::lowest::<Loan>(&[]), None);
}

#[test]
fn placing_keeps_the_order_and_lands_after_equals() {
    let sorted = vec![
        Loan { title: 1, days: 2 },
        Loan { title: 2, days: 5 },
        Loan { title: 3, days: 9 },
    ];
    let put = patterns::placed(&sorted, Loan { title: 9, days: 5 });
    let titles: Vec<u32> = put.iter().map(|l| l.title).collect();
    assert_eq!(titles, vec![1, 2, 9, 3]);
    let tail = patterns::placed(&sorted, Loan { title: 9, days: 10 });
    assert_eq!(tail.last().unwrap().title, 9);
    let head = patterns::placed(&sorted, Loan { title: 9, days: 1 });
    assert_eq!(head[0].title, 9);
}

#[test]
fn ordering_sorts_by_rank_and_is_stable() {
    let titles: Vec<u32> = patterns::ordered(&stack()).iter().map(|l| l.title).collect();
    assert_eq!(titles, vec![3, 1, 5, 2, 4]);
}

#[test]
fn a_window_is_closed_below_and_open_above() {
    let titles: Vec<u32> = patterns::keep_between(&stack(), 5, 9)
        .iter()
        .map(|l| l.title)
        .collect();
    assert_eq!(titles, vec![1, 5]);
    assert!(patterns::keep_between(&stack(), 9, 9).is_empty());
}

#[test]
fn splitting_gives_two_complementary_halves_in_order() {
    let (over, under) = patterns::split_above(&stack(), 5);
    let up: Vec<u32> = over.iter().map(|l| l.title).collect();
    let down: Vec<u32> = under.iter().map(|l| l.title).collect();
    assert_eq!(up, vec![2, 4, 5]);
    assert_eq!(down, vec![1, 3]);
}

#[test]
fn dequeuing_takes_the_front_and_leaves_the_rest() {
    let (head, rest) = patterns::dequeued(&[4, 5, 6]).unwrap();
    assert_eq!(head, 4);
    assert_eq!(rest, vec![5, 6]);
    assert_eq!(patterns::dequeued::<u32>(&[]), None);
}

#[test]
fn rotating_moves_the_front_to_the_back() {
    assert_eq!(patterns::rotated(&[1, 2, 3]), vec![2, 3, 1]);
    assert_eq!(patterns::rotated(&[7]), vec![7]);
    assert_eq!(patterns::rotated::<u32>(&[]), Vec::<u32>::new());
}

#[test]
fn moving_leaves_one_shelf_and_arrives_on_the_other() {
    let shelf = vec![Loan { title: 1, days: 5 }, Loan { title: 2, days: 9 }];
    let desk = vec![Loan { title: 7, days: 1 }];
    let (left, arrived) = patterns::moved(&shelf, &desk, 2);
    assert_eq!(left, vec![Loan { title: 1, days: 5 }]);
    assert_eq!(
        arrived,
        vec![Loan { title: 7, days: 1 }, Loan { title: 2, days: 9 }]
    );
    let (same_left, same_arrived) = patterns::moved(&shelf, &desk, 3);
    assert_eq!(same_left, shelf);
    assert_eq!(same_arrived, desk);
}

#[test]
fn replacing_keeps_the_position_and_ignores_an_absent_key() {
    let out = patterns::replaced(&stack(), Loan { title: 3, days: 50 });
    assert_eq!(out[2], Loan { title: 3, days: 50 });
    assert_eq!(out.len(), 5);
    assert_eq!(out[0], Loan { title: 1, days: 5 });
    assert_eq!(patterns::replaced(&stack(), Loan { title: 9, days: 1 }), stack());
}

#[test]
fn a_page_is_a_window_by_number_and_a_page_past_the_end_is_empty() {
    let items = [1, 2, 3, 4, 5];
    assert_eq!(patterns::page(&items, 2, 0), vec![1, 2]);
    assert_eq!(patterns::page(&items, 2, 2), vec![5]);
    assert_eq!(patterns::page(&items, 2, 3), Vec::<u32>::new());
    assert_eq!(patterns::page(&items, 0, 0), Vec::<u32>::new());
    assert_eq!(patterns::page(&items, usize::MAX, 2), Vec::<u32>::new());
}

#[test]
fn a_tally_counts_each_facet_value_once_in_first_seen_order() {
    assert_eq!(patterns::tally::<ByLine, Ticket>(&desk()), vec![(1, 2), (2, 1)]);
    assert_eq!(patterns::tally::<ByLine, Ticket>(&[]), vec![]);
}

#[test]
fn a_running_total_shows_the_balance_after_every_step() {
    assert_eq!(patterns::running_total::<ByDays, Loan>(&stack(), 10), vec![15, 24, 26, 35, 42]);
    assert_eq!(
        patterns::running_total::<ByDays, Loan>(&[Loan { title: 1, days: u32::MAX }], 1),
        vec![u32::MAX]
    );
}

#[test]
fn a_share_truncates_and_survives_a_product_that_would_overflow() {
    assert_eq!(patterns::share_of(250u32, 30, 100), Some(75));
    assert_eq!(patterns::share_of(7u32, 1, 3), Some(2));
    assert_eq!(patterns::share_of(3_000_000_000u32, 50, 100), Some(1_500_000_000));
    assert_eq!(patterns::share_of(5u32, 1, 0), None);
}

#[test]
fn ceiling_division_rounds_up_only_when_something_is_left() {
    assert_eq!(patterns::div_up(9u32, 4), Some(3));
    assert_eq!(patterns::div_up(8u32, 4), Some(2));
    assert_eq!(patterns::div_up(0u32, 4), Some(0));
    assert_eq!(patterns::div_up(9u32, 0), None);
}

#[test]
fn backoff_scales_until_it_meets_the_cap() {
    assert_eq!(patterns::backoff(3u32, 2, 10), 6);
    assert_eq!(patterns::backoff(6u32, 2, 10), 10);
    assert_eq!(patterns::backoff(u32::MAX, 2, u32::MAX), u32::MAX);
}

#[test]
fn a_window_test_accepts_the_floor_and_refuses_the_roof() {
    assert!(patterns::is_within(3, 3, 5));
    assert!(patterns::is_within(4, 3, 5));
    assert!(!patterns::is_within(5, 3, 5));
    assert!(!patterns::is_within(2, 3, 5));
}

#[test]
fn a_spend_fits_up_to_the_cap_and_an_overflow_does_not_read_as_room() {
    assert!(patterns::fits(7u32, 3, 10));
    assert!(!patterns::fits(7u32, 4, 10));
    assert!(!patterns::fits(u32::MAX, 1, 10));
}

#[test]
fn something_is_due_once_the_interval_has_elapsed() {
    assert!(!patterns::is_due(10u32, 7, 16));
    assert!(patterns::is_due(10u32, 7, 17));
    assert!(patterns::is_due(10u32, 7, 30));
}

#[test]
fn cycling_wraps_into_the_range_and_an_empty_range_is_none() {
    assert_eq!(patterns::cycled(9u32, 0, 7), Some(2));
    assert_eq!(patterns::cycled(25u32, 0, 24), Some(1));
    assert_eq!(patterns::cycled(7u32, 3, 7), Some(3));
    assert_eq!(patterns::cycled(1u32, 3, 7), Some(3));
    assert_eq!(patterns::cycled(5u32, 5, 5), None);
}

trait Guards {
    fn too_many(&self, asked: &u32) -> bool;
    fn would_panic(&self, asked: &u32) -> bool;
}

impl Guards for u32 {
    fn too_many(&self, asked: &u32) -> bool {
        *self + *asked > 3
    }
    fn would_panic(&self, _asked: &u32) -> bool {
        panic!("a guard after a refusal was evaluated")
    }
}

#[test]
fn the_lazy_guard_table_stops_at_the_first_refusal_and_reads_the_command() {
    let table: &[Guard<u32, u32, &str>] = &[
        (<u32 as Guards>::too_many, "too many"),
        (<u32 as Guards>::would_panic, "never"),
    ];
    assert_eq!(refuse_when_lazily(&2, &2, table, "ok"), Err("too many"));
    let none: &[Guard<u32, u32, &str>] = &[(<u32 as Guards>::too_many, "too many")];
    assert_eq!(refuse_when_lazily(&1, &1, none, "ok"), Ok("ok"));
}

#[test]
fn ordering_a_large_list_finishes_and_keeps_arrival_order_among_equals() {
    let many: Vec<Loan> = (0..20_000u32)
        .map(|n| Loan { title: n, days: (20_000 - n) / 4 })
        .collect();
    let sorted = patterns::ordered(&many);
    assert_eq!(sorted.len(), many.len());
    for pair in sorted.windows(2) {
        assert!(pair[0].days <= pair[1].days);
        if pair[0].days == pair[1].days {
            assert!(pair[0].title < pair[1].title, "equal ranks lost arrival order");
        }
    }
    assert_eq!(sorted[0].days, 0);
    assert_eq!(sorted.last().unwrap().title, 0);
}

#[test]
fn cycling_at_the_unsigned_edges_lands_on_the_floor() {
    assert_eq!(patterns::cycled(2u32, 3, 7), Some(3));
    assert_eq!(patterns::cycled(3u32, 3, 7), Some(3));
    assert_eq!(patterns::cycled(7u32, 3, 7), Some(3));
    assert_eq!(patterns::cycled(11u32, 3, 7), Some(3));
    assert_eq!(patterns::cycled(6u32, 3, 7), Some(6));
    assert_eq!(patterns::cycled(0u32, 0, 0), None);
    assert_eq!(patterns::cycled(9u32, 7, 3), None);
}

#[test]
fn ceiling_division_at_the_unsigned_edges() {
    assert_eq!(patterns::div_up(0u32, 1), Some(0));
    assert_eq!(patterns::div_up(4u32, 4), Some(1));
    assert_eq!(patterns::div_up(5u32, 4), Some(2));
    assert_eq!(patterns::div_up(1u32, 4), Some(1));
    assert_eq!(patterns::div_up(u32::MAX, 1), Some(u32::MAX));
    assert_eq!(patterns::div_up(u32::MAX, 2), Some(2_147_483_648));
    assert_eq!(patterns::div_up(1u32, 0), None);
    assert_eq!(patterns::div_up(0u32, 0), None);
}

#[test]
fn moving_takes_every_element_under_a_shared_key() {
    let shelf = vec![
        Loan { title: 2, days: 5 },
        Loan { title: 1, days: 1 },
        Loan { title: 2, days: 9 },
    ];
    let (left, arrived) = patterns::moved(&shelf, &[], 2);
    assert_eq!(left, vec![Loan { title: 1, days: 1 }]);
    assert_eq!(
        arrived,
        vec![Loan { title: 2, days: 5 }, Loan { title: 2, days: 9 }]
    );
}

#[test]
fn replacing_rewrites_every_element_under_a_shared_key() {
    let shelf = vec![
        Loan { title: 2, days: 5 },
        Loan { title: 1, days: 1 },
        Loan { title: 2, days: 9 },
    ];
    let out = patterns::replaced(&shelf, Loan { title: 2, days: 0 });
    assert_eq!(
        out,
        vec![
            Loan { title: 2, days: 0 },
            Loan { title: 1, days: 1 },
            Loan { title: 2, days: 0 },
        ]
    );
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

fn u16_at(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

const FRAME: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

#[test]
fn a_saved_frame_carries_the_header_a_viewer_reads() {
    let out = bitmap(&FRAME, 2, 2);
    assert_eq!(out[0], b'B');
    assert_eq!(out[1], b'M');
    assert_eq!(u32_at(&out, 2), out.len() as u32);
    assert_eq!(u32_at(&out, 10), 54);
    assert_eq!(u32_at(&out, 14), 40);
    assert_eq!(u32_at(&out, 18), 2);
    assert_eq!(u32_at(&out, 22), 2);
    assert_eq!(u16_at(&out, 26), 1);
    assert_eq!(u16_at(&out, 28), 24);
    assert_eq!(u32_at(&out, 30), 0);
}

#[test]
fn a_saved_frame_runs_bottom_upward_in_blue_green_red() {
    let out = bitmap(&FRAME, 2, 2);
    assert_eq!(&out[54..], &[9, 8, 7, 12, 11, 10, 0, 0, 3, 2, 1, 6, 5, 4, 0, 0]);
}

#[test]
fn a_row_that_does_not_end_on_the_block_is_padded() {
    let out = bitmap(&FRAME, 1, 1);
    assert_eq!(out.len(), 54 + 4);
    assert_eq!(&out[54..], &[3, 2, 1, 0]);
    assert_eq!(u32_at(&out, 34), 4);
}

#[test]
fn a_frame_wide_enough_to_need_no_padding_gets_none() {
    let out = bitmap(&FRAME, 4, 1);
    assert_eq!(out.len(), 54 + 12);
}

#[test]
fn alpha_is_dropped_and_the_colours_keep_their_order() {
    assert_eq!(without_alpha(&[1, 2, 3, 255, 4, 5, 6, 255]), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn a_frame_from_the_other_corner_is_turned_over() {
    assert_eq!(rows_flipped(&[1, 2, 3, 4], 2), vec![3, 4, 1, 2]);
    assert_eq!(rows_flipped(&[1, 2], 0), Vec::<u8>::new());
}

#[test]
fn a_shot_name_sorts_in_the_order_it_was_made() {
    assert_eq!(numbered("shot-", 7, 4), "shot-0007");
    assert_eq!(numbered("shot-", 1234, 4), "shot-1234");
    assert!(numbered("shot-", 9, 4) < numbered("shot-", 10, 4));
}

#[test]
fn a_shot_reaches_the_disk_and_says_so_when_it_cannot() {
    let path = std::env::temp_dir().join(format!("premise_shot_{}.bmp", std::process::id()));
    let named = path.to_string_lossy().to_string();
    assert!(shot(&named, &FRAME, 2, 2).is_ok());
    assert_eq!(std::fs::read(&path).unwrap(), bitmap(&FRAME, 2, 2));
    let _ = std::fs::remove_file(&path);
    assert!(shot("", &FRAME, 2, 2).is_err());
}

#[test]
fn two_is_reachable_without_writing_it() {
    assert_eq!(doubled(21u32), 42);
    assert_eq!(half(42u32), 21);
    assert_eq!(half(7u32), 3);
    assert_eq!(doubled(u8::MAX), u8::MAX);
    assert_eq!(half(0u32), 0);
}

#[test]
fn a_distance_does_not_ask_which_side_is_larger() {
    assert_eq!(away(9u32, 4), 5);
    assert_eq!(away(4u32, 9), 5);
    assert_eq!(away(4u32, 4), 0);
}

#[test]
fn a_flag_turns_over() {
    assert!(flipped(false));
    assert!(!flipped(true));
}

#[test]
fn a_value_folds_into_a_window_by_whole_steps() {
    assert_eq!(folded(75u32, 60, 12), Some(63));
    assert_eq!(folded(63u32, 60, 12), Some(63));
    assert_eq!(folded(60u32, 60, 12), Some(60));
    assert_eq!(folded(5u32, 60, 12), Some(60));
    assert_eq!(folded(9u32, 0, 0), None);
}

#[test]
fn a_position_that_is_not_there_answers_with_nothing() {
    assert_eq!(at(&[7u32, 8, 9], 1), Some(8));
    assert_eq!(at(&[7u32, 8, 9], 3), None);
    assert_eq!(at::<u32>(&[], 0), None);
}

#[test]
fn a_side_of_every_pair_comes_out_on_its_own() {
    let pairs = vec![(1u32, "a"), (2, "b")];
    assert_eq!(firsts(&pairs), vec![1, 2]);
    assert_eq!(seconds(&pairs), vec!["a", "b"]);
}

#[test]
fn a_point_between_two_values_runs_both_ways() {
    assert_eq!(between(0u32, 100, 1, 4), Some(25));
    assert_eq!(between(100u32, 0, 1, 4), Some(75));
    assert_eq!(between(10u32, 20, 0, 4), Some(10));
    assert_eq!(between(10u32, 20, 4, 4), Some(20));
    assert_eq!(between(10u32, 20, 1, 0), None);
}

#[test]
fn the_closest_candidate_wins_and_a_tie_keeps_the_first() {
    assert_eq!(nearest(10u32, &[1, 9, 30]), Some(9));
    assert_eq!(nearest(10u32, &[9, 11]), Some(9));
    assert_eq!(nearest(10u32, &[11, 9]), Some(11));
    assert_eq!(nearest(10u32, &[]), None);
}

#[test]
fn work_splits_into_pieces_no_larger_than_asked_for() {
    assert_eq!(parted(&[1u32, 2, 3, 4, 5], 2), vec![&[1u32, 2, 3][..], &[4, 5][..]]);
    assert_eq!(parted(&[1u32, 2], 5).len(), 2);
    assert_eq!(parted(&[1u32, 2], 0).len(), 0);
    assert_eq!(parted::<u32>(&[], 4).len(), 0);
    assert!(parted(&[1u32, 2, 3, 4, 5], 3).len() <= 3);
}

fn squared(n: &u32) -> u32 {
    n * n
}

#[test]
fn work_over_the_machine_answers_in_the_order_it_was_given() {
    let items: Vec<u32> = (0..1000).collect();
    let done = across(&items, squared);
    assert_eq!(done.len(), items.len());
    assert_eq!(done, items.iter().map(squared).collect::<Vec<u32>>());
    assert_eq!(across::<u32, u32>(&[], squared), Vec::<u32>::new());
    assert!(lanes_here() >= 1);
}
