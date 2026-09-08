use patterns::because;

pub trait Cap {
    const MAX: u32;
}
pub struct Plan;
because!(Plan, "the plan a member is on, which fixes how many books they may hold");
impl Cap for Plan {
    const MAX: u32 = 500;
}
