use patterns::{because, Bounded};

pub const SLOT_CAP: u32 = 20;
because!(SLOT_CAP, "a full backpack row on the narrowest supported screen");

pub struct Slots;
because!(Slots, "a slot count, not a quantity, so the bound belongs to the type not the value");

impl Bounded for Slots {
    type Of = u32;
    const LO: u32 = 0;
    const HI: u32 = SLOT_CAP;
}

pub const HELD_LABEL: &str = "held";
because!(HELD_LABEL, "the key the inventory HUD reads when it redraws the slot counter");

pub const CAP_LABEL: &str = "cap";
because!(CAP_LABEL, "the key the HUD reads for the ceiling it draws the bar against");

pub const SEPARATOR: &str = " ";
because!(SEPARATOR, "a single space is what the fixed width HUD row uses");

pub const MIN_DAMAGE: u32 = 1;
because!(MIN_DAMAGE, "armor never grants full immunity in playtest");
