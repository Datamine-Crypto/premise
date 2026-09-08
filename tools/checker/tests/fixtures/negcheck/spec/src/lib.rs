use patterns::because;

pub const PENCE: u32 = 15;
because!(PENCE, "what an overdue day costs, set against the cost of chasing it");
const GAP: u32 = PENCE - 15;
because!(GAP, "the gap between the charge and the figure the desk quoted");
const HOLDS: bool = !(GAP != 0);
because!(HOLDS, "the check that the charge and the quoted figure agree");
const _: () = assert!(HOLDS);
