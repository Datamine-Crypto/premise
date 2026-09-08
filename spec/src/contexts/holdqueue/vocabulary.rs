use patterns::{because, provisional, source, Named, Valued};

pub struct LendingPolicy2026;
source!(LendingPolicy2026, "the branch policy in force, which allows one queue per member per title");

pub struct HoldQueue;
because!(
    HoldQueue,
    LendingPolicy2026,
    "the branch hold shelf, the one queue a member may join for a title"
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

#[derive(Clone, Debug, PartialEq)]
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

pub struct ByAllowance;

pub const COLLECT_DAYS: u32 = 7;
provisional!(
    COLLECT_DAYS,
    "chosen as one whole visit cycle so a member on a fixed weekday gets one chance at the shelf, but no branch has confirmed its cycle; the desk supervisor would settle it"
);

pub const MEMBER_HOLDS: usize = 5;
provisional!(
    MEMBER_HOLDS,
    "chosen as what the desk can find by hand under one member name, but nobody has measured the hold shelf; the branch would settle it"
);

pub const STAFF_EXTRA: usize = 5;
provisional!(
    STAFF_EXTRA,
    "chosen because staff queue for reading groups as well as themselves, but no staffing policy has been quoted; the branch would settle it"
);

pub const STAFF_HOLDS: usize = MEMBER_HOLDS + STAFF_EXTRA;

const _: () = assert!(STAFF_HOLDS > MEMBER_HOLDS);

impl Valued<ByAllowance> for Standing {
    type Value = usize;

    fn value(&self) -> usize {
        match self {
            Standing::Member => MEMBER_HOLDS,
            Standing::Staff => STAFF_HOLDS,
        }
    }
}
