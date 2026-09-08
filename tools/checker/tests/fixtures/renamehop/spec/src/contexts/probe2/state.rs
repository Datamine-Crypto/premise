use crate::contexts::probe::vocabulary as fv;
use fv as fw;
use patterns::Context;

type FS = <fw::Probe as Context>::State;

pub struct State {
    pub a: Option<FS>,
}
