use patterns::{because, provisional, source, Named, Valued};

pub struct ForecourtParkingReview;
source!(
    ForecourtParkingReview,
    "the station review of forecourt cycle parking that set what a locker bay is worth and how long an unclaimed one may stand"
);

pub struct Lockers;
because!(
    Lockers,
    ForecourtParkingReview,
    "the bank of cycle lockers on the forecourt, each let to one renter at a time"
);

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Compact,
    Standard,
    Cargo,
}

impl Named for Size {
    fn text(&self) -> &'static str {
        match self {
            Size::Compact => "compact",
            Size::Standard => "standard",
            Size::Cargo => "cargo",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Contents {
    Empty,
    Cycle,
    Gear,
}

impl Named for Contents {
    fn text(&self) -> &'static str {
        match self {
            Contents::Empty => "empty",
            Contents::Cycle => "cycle",
            Contents::Gear => "gear",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Moved {
    Held,
    Passed,
}

impl Named for Moved {
    fn text(&self) -> &'static str {
        match self {
            Moved::Held => "held",
            Moved::Passed => "passed",
        }
    }
}

pub enum Command {
    Rent {
        locker: u32,
        renter: u32,
        size: Size,
    },
    Transfer {
        locker: u32,
        from: u32,
        to: u32,
    },
    Reclaim {
        locker: u32,
        contents: Contents,
    },
    PassDay,
}

pub enum Event {
    Rented {
        locker: u32,
        renter: u32,
        size: Size,
    },
    Transferred {
        locker: u32,
        renter: u32,
        size: Size,
        left: u32,
    },
    Reclaimed {
        locker: u32,
        renter: u32,
        contents: Contents,
    },
    DayPassed,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    AlreadyRented,
    Full,
    NotRented,
    NotTheRenter,
    AlreadyTransferred,
    WithinTerm,
}

pub struct ByRenter;

pub struct ByHolder;

pub struct ByAbandoned;

pub struct BySize;

pub struct ByContents;

pub struct ByRate;

pub const STANDARD_PENCE_PER_DAY: u32 = 60;
because!(
    STANDARD_PENCE_PER_DAY,
    ForecourtParkingReview,
    "the daily rate the review set for a bay that takes an ordinary commuting bicycle, being where the bank covers its own upkeep and warden time"
);

pub const COMPACT_SAVING: u32 = 20;
because!(
    COMPACT_SAVING,
    ForecourtParkingReview,
    "what comes off the standard rate for a bay too short to take a full frame, enough to keep the small units let rather than standing empty"
);

pub const COMPACT_PENCE_PER_DAY: u32 = STANDARD_PENCE_PER_DAY - COMPACT_SAVING;

pub const CARGO_BAYS: u32 = 2;
because!(
    CARGO_BAYS,
    ForecourtParkingReview,
    "how many ordinary bays one cargo cycle takes out of the bank, which is what the renter of that locker pays for"
);

pub const CARGO_PENCE_PER_DAY: u32 = STANDARD_PENCE_PER_DAY * CARGO_BAYS;

const _: () = assert!(COMPACT_PENCE_PER_DAY < STANDARD_PENCE_PER_DAY);

const _: () = assert!(CARGO_PENCE_PER_DAY > STANDARD_PENCE_PER_DAY);

pub const LOCKER_TERM_DAYS: u32 = 90;
because!(
    LOCKER_TERM_DAYS,
    ForecourtParkingReview,
    "the length of let the review chose to match the season ticket the station already sells, so a renter renews both on one visit"
);

pub const LOCKER_GRACE_DAYS: u32 = 14;
because!(
    LOCKER_GRACE_DAYS,
    ForecourtParkingReview,
    "how long staff wait past the end of a let before cutting the lock, long enough to cover a renter away on holiday and short enough that a bay is not held all season"
);

impl Valued<ByRate> for Size {
    type Value = u32;

    fn value(&self) -> u32 {
        match self {
            Size::Compact => COMPACT_PENCE_PER_DAY,
            Size::Standard => STANDARD_PENCE_PER_DAY,
            Size::Cargo => CARGO_PENCE_PER_DAY,
        }
    }
}

pub const LOCKER_BAYS: usize = 24;
provisional!(
    LOCKER_BAYS,
    ForecourtParkingReview,
    "how many bays the forecourt holds, which the review counted without giving the figure; the station facilities desk would settle it"
);
