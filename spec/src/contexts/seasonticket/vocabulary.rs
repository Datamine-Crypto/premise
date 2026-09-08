use patterns::{because, source, supersedes, Named, Valued};

pub struct ParkingOrder;
source!(
    ParkingOrder,
    "the traffic order that governs the car park, which fixes the season a bay is let for and the days a holder may stand a let down while away"
);

pub struct ConcessionAgreement;
source!(
    ConcessionAgreement,
    "the agreement with the estate that first priced a move to another bay, and which the scheme still honours for a holder who bought under it"
);

pub struct TariffReview;
source!(
    TariffReview,
    "the review of scheme charges against operating cost that repriced a move to another bay and named the day the new price applied from"
);

pub struct SeasonTicket;
because!(
    SeasonTicket,
    ParkingOrder,
    "the let of one named bay to one holder for a season, the only thing the order lets the scheme sell"
);

#[derive(Clone, Copy, PartialEq)]
pub enum Tier {
    Resident,
    Visitor,
}

impl Named for Tier {
    fn text(&self) -> &'static str {
        match self {
            Tier::Resident => "resident",
            Tier::Visitor => "visitor",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cover {
    Active,
    Suspended,
}

impl Named for Cover {
    fn text(&self) -> &'static str {
        match self {
            Cover::Active => "active",
            Cover::Suspended => "suspended",
        }
    }
}

pub enum Command {
    Buy { ticket: u32, bay: u32, tier: Tier },
    Suspend { ticket: u32 },
    Resume { ticket: u32 },
    Reassign { ticket: u32, bay: u32 },
    Surrender { ticket: u32 },
    PassDay,
}

pub enum Event {
    Bought { ticket: u32, bay: u32, tier: Tier },
    Suspended { ticket: u32, since: u32 },
    Resumed { ticket: u32, granted: u32 },
    Reassigned { ticket: u32, bay: u32, fee: u32 },
    Surrendered { ticket: u32 },
    DayPassed,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    AlreadyIssued,
    NoTicket,
    BayTaken,
    AlreadySuspended,
    NotSuspended,
    AllowanceSpent,
    Lapsed,
    SameBay,
}

pub struct ByBay;

pub struct ByStandDown;

pub const DAYS_PER_WEEK: u32 = 7;
because!(
    DAYS_PER_WEEK,
    ParkingOrder,
    "the week the scheme calendar counts in, so a season and a stand down both end on the weekday they started"
);

pub const TERM_WEEKS: u32 = 13;
because!(
    TERM_WEEKS,
    ParkingOrder,
    "the quarter the order bills a season in, so a holder renews on one fixed weekday and the desk reads one renewal list a quarter"
);

pub const TERM_DAYS: u32 = TERM_WEEKS * DAYS_PER_WEEK;

pub const RESIDENT_SUSPEND_WEEKS: u32 = 4;
because!(
    RESIDENT_SUSPEND_WEEKS,
    ParkingOrder,
    "the longest absence the order expects of someone who lives on the estate, taken from the leave a full time worker there can book in one quarter"
);

pub const RESIDENT_SUSPEND_DAYS: u32 = RESIDENT_SUSPEND_WEEKS * DAYS_PER_WEEK;

pub const VISITOR_SUSPEND_WEEKS: u32 = 1;
because!(
    VISITOR_SUSPEND_WEEKS,
    ParkingOrder,
    "a visitor bay pays for itself only if it turns over, so the order allows a stand down long enough to cover a trip away and no longer"
);

pub const VISITOR_SUSPEND_DAYS: u32 = VISITOR_SUSPEND_WEEKS * DAYS_PER_WEEK;

const _: () = assert!(RESIDENT_SUSPEND_DAYS > VISITOR_SUSPEND_DAYS);
const _: () = assert!(TERM_DAYS > RESIDENT_SUSPEND_DAYS);

pub const LEGACY_REASSIGN_PENCE: u32 = 250;
because!(
    LEGACY_REASSIGN_PENCE,
    ConcessionAgreement,
    "the price that agreement fixed for moving a holder to another bay, still owed by a holder who bought a season under it"
);

pub const REASSIGN_PENCE: u32 = 400;
because!(
    REASSIGN_PENCE,
    TariffReview,
    "the price the review set against the cost of re signing a bay and re issuing a permit to the holder"
);
supersedes!(
    REASSIGN_PENCE,
    LEGACY_REASSIGN_PENCE,
    TARIFF_DAY,
    "the review that reset what a move to another bay costs"
);

const _: () = assert!(REASSIGN_PENCE > LEGACY_REASSIGN_PENCE);

pub const TARIFF_DAY: u32 = 730;
because!(
    TARIFF_DAY,
    TariffReview,
    "the day of the scheme calendar the new price applied from, so a season sold before it is charged at the price it was sold under"
);

impl Valued<ByStandDown> for Tier {
    type Value = u32;

    fn value(&self) -> u32 {
        match self {
            Tier::Resident => RESIDENT_SUSPEND_DAYS,
            Tier::Visitor => VISITOR_SUSPEND_DAYS,
        }
    }
}
