use patterns::because;

pub struct Probe;
because!(Probe, Bar, Cap, "the probe context, whose bar and cap the two tables below decide");
pub trait Bar {
    const BAR: u32;
}
pub trait Cap {
    const CAP: u32;
}
impl Bar for Probe {
    const BAR: u32 = 250;
}
impl Cap for Probe {
    const CAP: u32 = 500;
}
