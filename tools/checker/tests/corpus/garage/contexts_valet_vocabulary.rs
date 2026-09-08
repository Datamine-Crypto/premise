use patterns::{because};

pub struct Valet;
because!(Valet, "the attended service the deck ran until the 2023 staffing review");

pub enum Command { Hand, Fetch }
pub enum Event { Handed, Fetched }
pub enum Fault { NoKey, NoDriver }

pub const KEY_SLOTS: u32 = 40;
because!(KEY_SLOTS, "the key cabinet the 2023 staffing review left in the booth holds this many");
