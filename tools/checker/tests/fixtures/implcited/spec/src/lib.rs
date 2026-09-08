use patterns::{because, decided};

pub trait Cap {
    const MAX: u32;
}
because!(Cap, "the ceiling a plan puts on what one member may hold at once");
pub struct Plan;
because!(Plan, "the plan a member is on, which fixes how many books they may hold");
decided!(Plan, Cap, "the ceiling the branch settled on for the ordinary plan");
impl Cap for Plan {
    const MAX: u32 = 500;
}
