use patterns::{because, Bounded};

pub const BUCKET_CAPACITY: u32 = 60;
because!(BUCKET_CAPACITY, "p99 burst measured at 58 requests per minute");

pub const REFILL_PER_TICK: u32 = 1;
because!(REFILL_PER_TICK, "one token per second sustains the measured mean");

pub struct Tokens;

impl Bounded for Tokens {
    type Of = u32;
    const LO: u32 = 0;
    const HI: u32 = BUCKET_CAPACITY;
}
