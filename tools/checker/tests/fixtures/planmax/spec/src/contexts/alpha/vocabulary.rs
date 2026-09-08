use patterns::{because, decided};

pub trait Slots {
    const MAX: u32;
}
pub struct Plan;
because!(Plan, "the plan a member is on, which fixes how many books they may hold");
impl Slots for Plan {
    const MAX: u32 = 4;
}
pub const Plan_MAX: u32 = 4;
because!(Plan_MAX, "the same figure written a second time under the name it crosses as");
decided!(Plan, Slots, "the slot count the branch settled on for the ordinary plan");
