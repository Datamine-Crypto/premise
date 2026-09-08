use patterns::Context;

type F = crate::contexts::probe::vocabulary::Probe;

pub struct State {
    pub a: <F as Context>::State,
}
