use patterns::because;

pub const PENCE: u32 = 15;
because!(PENCE, "what an overdue day costs, set against the cost of chasing it");
const PINNED: () = assert!(PENCE == 15);
because!(PINNED, "the assertion that pins the daily charge where nothing else states it");
