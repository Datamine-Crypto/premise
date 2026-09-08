use super::vocabulary::{Pick, Probe2};
use patterns::Context;

pub struct State {
    pub b: <<Probe2 as Pick>::M as Context>::State,
}
