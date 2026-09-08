use patterns::because;

pub const PENCE: u32 = 15;
because!(PENCE, "what an overdue day costs, set against the cost of chasing it");
const PINNED: u32 = PENCE - 15;
because!(PINNED, "the gap between the charge and the figure the desk quoted");
const _: () = assert!(PINNED == 0);
