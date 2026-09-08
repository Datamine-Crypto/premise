use crate::contexts::alpha::vocabulary::{Floor, Probe};
use patterns::decided;

pub struct State {
    pub level: u32,
}

decided!(Probe, Floor, "the floor the desk settled on for the ordinary probe");
impl Floor for Probe {
    const FLOOR: u32 = 3;
}
