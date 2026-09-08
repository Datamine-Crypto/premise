use patterns::{because};

pub struct Dock;
because!(Dock, "the street furniture the council maintains under a separate contract");

pub enum Command { Lock, Release }
pub enum Event { Locked, Released }
pub enum Fault { BayFull, BayEmpty }

pub enum Status { Idle, Active, Faulted }
because!(Status, "a dock is faulted when the lock jams, which no hire can observe");

pub const BAY_COUNT: u32 = 24;
because!(BAY_COUNT, "the 2025 street survey fitted this many bays to the standard kerb run");
