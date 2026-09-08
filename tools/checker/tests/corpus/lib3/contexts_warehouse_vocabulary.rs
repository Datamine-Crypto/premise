use patterns::{because};

pub struct Warehouse;
because!(Warehouse, "the offsite store the 2025 relocation created");

pub enum Command { Pick, Shelve }
pub enum Event { Picked, Shelved }
pub enum Fault { NotHeld, BinFull }

pub const PICK_DAYS: u32 = 4;
because!(PICK_DAYS, "the 2025 relocation contract guarantees retrieval inside four working days");

pub const BIN_CAP: u32 = 60;
because!(BIN_CAP, "the 2025 relocation fitted sixty slots to each aisle bay");

