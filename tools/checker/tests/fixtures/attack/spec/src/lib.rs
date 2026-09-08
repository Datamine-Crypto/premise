use patterns::Context;

pub const A: u32 = 4;

pub const LOOPED: u32 = {
    let mut i = 0u32;
    while i < A {
        i += A;
    }
    i
};

pub struct Beta;

impl Context for Beta {
    type State = u32;
    type Command = u32;
    type Event = u32;
    type Fault = u32;

    fn initial() -> u32 {
        0
    }

    fn decide(_s: &u32, c: &u32) -> Result<u32, u32> {
        match hot(c) {
            true => Ok(0),
            false => Err(1),
        }
    }

    fn apply(s: u32, e: &u32) -> u32 {
        match e {
            0 => s,
            _ => s,
        }
    }
}
