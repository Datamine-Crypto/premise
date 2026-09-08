use patterns::because;

pub trait Caps {
    const LO: u32 = 15;
    const HI: u32 = 500;
}
because!(Caps, "the floor and the ceiling of a probe charge, both settled by the desk at once");
