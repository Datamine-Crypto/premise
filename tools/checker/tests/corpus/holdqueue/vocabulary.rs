use patterns::{because, Named, Ranked};

pub struct HoldQueue;
because!(
    HoldQueue,
    "the branch hold shelf, the one queue the 2026 lending policy lets a member join"
);

#[derive(Clone, Copy, PartialEq)]
pub enum Standing {
    Member,
    Staff,
}

impl Named for Standing {
    fn text(&self) -> &'static str {
        match self {
            Standing::Member => "member",
            Standing::Staff => "staff",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Ready {
    Waiting,
    Notified,
}

impl Named for Ready {
    fn text(&self) -> &'static str {
        match self {
            Ready::Waiting => "waiting",
            Ready::Notified => "notified",
        }
    }
}

pub enum Command {
    Place {
        title: u32,
        member: u32,
        standing: Standing,
    },
    Available {
        title: u32,
    },
    Collect {
        title: u32,
        member: u32,
    },
    PassDay,
}

pub enum Event {
    Placed { title: u32, member: u32 },
    Notified { title: u32, member: u32 },
    Collected { title: u32, member: u32 },
    DayPassed,
}

#[derive(Debug, PartialEq)]
pub enum Fault {
    AlreadyQueued,
    AtAllowance,
    NoQueue,
    AlreadyOffered,
    NoClaim,
}

pub struct ByMember;

pub struct ByQueue;

pub struct ByClaim;

pub const COLLECT_DAYS: u32 = 7;
because!(
    COLLECT_DAYS,
    "7 days is one whole visit cycle, so a member who reaches the branch on a single fixed weekday still gets one chance at the shelf"
);

pub const MEMBER_HOLDS: usize = 5;
because!(
    MEMBER_HOLDS,
    "5 is the count of held titles the desk can still find by hand under one member name before the hold shelf needs a second pass"
);

pub const STAFF_EXTRA: usize = 5;
because!(
    STAFF_EXTRA,
    "5 more for staff, who queue titles for reading groups as well as themselves and who clear their own shelf without a desk visit"
);

pub const STAFF_HOLDS: usize = MEMBER_HOLDS + STAFF_EXTRA;

const _: () = assert!(STAFF_HOLDS > MEMBER_HOLDS);

impl Ranked for Standing {
    type Rank = usize;

    fn rank(&self) -> usize {
        match self {
            Standing::Member => MEMBER_HOLDS,
            Standing::Staff => STAFF_HOLDS,
        }
    }
}
