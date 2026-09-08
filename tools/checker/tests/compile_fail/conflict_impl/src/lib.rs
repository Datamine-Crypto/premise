use patterns::{Bounded, HasMax};

pub struct Hp;

impl Bounded for Hp {
    type Of = u32;
    const LO: u32 = 0;
    const HI: u32 = 100;
}

impl HasMax for Hp {
    type Of = u32;
    const MAX: u32 = 100;
}
