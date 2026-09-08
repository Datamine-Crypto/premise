use patterns::{because, source};

pub struct WearTest;
source!(WearTest, "the hinge fatigue test run on the entry door before fit-out");

pub struct Gate;
because!(Gate, WearTest, "the entry gate, the only fixture the test covered");

pub enum Command {
    Open,
    Lock,
}
pub enum Event {
    Opened,
    Locked,
}
#[derive(Clone)]
pub enum Fault {
    Worn,
    AlreadyLocked,
}

pub const OPEN_LIMIT: u32 = 3;
because!(OPEN_LIMIT, WearTest, "three openings is where the test showed hinge fatigue");
