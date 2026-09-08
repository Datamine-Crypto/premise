use patterns::because;

pub struct Window {
    pub days_after: u32,
    pub days_before: u32,
}
because!(Window, "the span either side of a due date within which a return counts as on time");
pub const AND_GATE: u32 = 3;
because!(AND_GATE, "the number of guards that must all pass before a rental opens");
