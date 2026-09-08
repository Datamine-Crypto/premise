use patterns::{because, Bounded};

pub struct Slots;

because!(Slots, "twenty slots fits the inventory grid without scrolling");

impl Bounded for Slots {
    type Of = u32;
    const LO: u32 = 0;
    const HI: u32 = 20;
}

because!(Ghost, "explains an item that does not exist");
