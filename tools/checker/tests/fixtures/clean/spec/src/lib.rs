use patterns::{because, Context};

pub const LIMIT: u32 = 42;
because!(LIMIT, "measured ceiling from the load test");

pub const HALF_LIMIT: u32 = LIMIT / 2;
because!(HALF_LIMIT, "the load test showed the ceiling holds only when the queue runs at half of it");

pub struct Gate;

pub enum Signal {
    Open(bool),
    Shut,
}

pub struct Flag {
    pub lit: bool,
}

impl Context for Gate {
    type State = Flag;
    type Event = Signal;

    fn initial() -> Flag {
        Flag { lit: false }
    }

    fn apply(state: Flag, event: &Signal) -> Flag {
        match event {
            Signal::Open(v) => Flag { lit: *v },
            Signal::Shut => Flag { lit: state.lit },
        }
    }
}
