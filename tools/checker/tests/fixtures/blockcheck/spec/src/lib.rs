use patterns::because;

pub const PENCE: u32 = 15;
because!(PENCE, "what an overdue day costs, set against the cost of chasing it");
const CHECK: u32 = { PENCE - 15 };
because!(CHECK, "the gap between the charge and the figure the desk quoted");
const _: () = assert!(CHECK == 0);
