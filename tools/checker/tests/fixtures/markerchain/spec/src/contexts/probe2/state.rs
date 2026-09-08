use patterns::Context;

type F1 = crate::contexts::probe::vocabulary::Probe;
type F2 = F1;

pub struct State {
    pub a: <F2 as Context>::State,
}
