use crate::contexts::shared::vocabulary::VehicleClass;
use patterns::{because};

pub struct Bay;
because!(Bay, "the surface deck, the only level the 2024 council survey measured");

pub enum Command { Enter(VehicleClass), Leave }
pub enum Event { Entered(VehicleClass), Left }
pub enum Fault { Full, Empty }

pub const DECK_BAYS: u32 = 120;
because!(DECK_BAYS, "the 2024 council survey counted this many marked bays on the surface deck");
