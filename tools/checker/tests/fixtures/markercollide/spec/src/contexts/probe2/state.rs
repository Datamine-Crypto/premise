use patterns::Context;

type F1 = crate::contexts::probe::vocabulary::Probe;
type F2 = F1;
type FS = <F2 as Context>::State;

pub struct State {
    pub a: FS,
}
