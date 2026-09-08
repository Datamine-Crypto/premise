use patterns::{because, provisional, source, supersedes, Named, Valued};

pub struct FineSchedule;
source!(
    FineSchedule,
    "the schedule of overdue charges the branch publishes to its members, which fixes what an overdue day costs, what one late loan may cost at most, who is charged nothing at all, and what a member may owe and still take a book out"
);

pub struct ChargesReview;
source!(
    ChargesReview,
    "the review of branch charges that repriced an overdue day and named the day of the lending calendar the new price ran from, leaving the old price owed on loans that fell overdue before it"
);

pub struct Fines;
because!(
    Fines,
    FineSchedule,
    "the charge a loan carries once it is past its due date, and the balance that charge leaves a member owing the branch"
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Grade {
    Junior,
    Adult,
}
because!(
    Grade,
    FineSchedule,
    "who the schedule charges: it waives an overdue charge for a junior member outright, so the grades differ by what a late loan may cost them and by nothing else"
);

impl Named for Grade {
    fn text(&self) -> &'static str {
        match self {
            Grade::Junior => "junior",
            Grade::Adult => "adult",
        }
    }
}

pub enum Command {
    Enrol { member: u32, grade: Grade },
    Borrow { loan: u32, member: u32 },
    Return { loan: u32 },
    Pay { member: u32, amount: u32 },
    PassDay,
}

pub enum Event {
    Enrolled { member: u32, grade: Grade },
    Borrowed { loan: u32, member: u32, due: u32 },
    Returned { loan: u32, member: u32, fine: u32 },
    Paid { member: u32, amount: u32 },
    DayPassed,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    AlreadyEnrolled,
    NotEnrolled,
    AlreadyOnLoan,
    NoLoan,
    Barred,
    NothingOwed,
}

pub struct ByBorrower;

pub struct ByCharge;

pub struct BySettle;

pub struct ByCap;

pub const LOAN_DAYS: u32 = 21;
provisional!(
    LOAN_DAYS,
    "how long a member keeps a book before it falls due; the schedule states a term but the branch has not given it, and the lending desk would settle it"
);

pub const LEGACY_PENCE_PER_DAY: u32 = 10;
provisional!(
    LEGACY_PENCE_PER_DAY,
    ChargesReview,
    "what an overdue day cost before the review, still owed on a loan that fell due under it; only the branch fee record can settle the figure"
);

pub const PENCE_PER_DAY: u32 = 15;
provisional!(
    PENCE_PER_DAY,
    ChargesReview,
    "what an overdue day costs since the review, set against what chasing a late return costs the desk; only the review minutes can settle the figure"
);
supersedes!(
    PENCE_PER_DAY,
    LEGACY_PENCE_PER_DAY,
    CHARGES_DAY,
    "the review that repriced an overdue day"
);

const _: () = assert!(PENCE_PER_DAY > LEGACY_PENCE_PER_DAY);

const _: () = assert!(LOAN_DAYS < CHARGES_DAY);

pub const CHARGES_DAY: u32 = 400;
provisional!(
    CHARGES_DAY,
    ChargesReview,
    "the day of the lending calendar the new rate ran from, before which a loan falling due stays on the old rate; the review minutes would settle the day"
);

pub const ADULT_FINE_CAP_PENCE: u32 = 500;
provisional!(
    ADULT_FINE_CAP_PENCE,
    "the most one late loan may cost a member the schedule charges, so a book left forgotten on a shelf stops growing a debt; the branch has not given the ceiling"
);

const _: () = assert!(ADULT_FINE_CAP_PENCE > PENCE_PER_DAY);

pub const BORROW_BAR_PENCE: u32 = 250;
provisional!(
    BORROW_BAR_PENCE,
    "the balance a member may owe and still take a book out, above which the desk lends nothing until it is paid down; the branch has not given the line"
);

impl Valued<ByCap> for Grade {
    type Value = u32;

    fn value(&self) -> u32 {
        match self {
            Grade::Junior => 0,
            Grade::Adult => ADULT_FINE_CAP_PENCE,
        }
    }
}
