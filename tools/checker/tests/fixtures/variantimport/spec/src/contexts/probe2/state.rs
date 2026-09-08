use crate::contexts::probe::vocabulary as pv;
use crate::contexts::probe::vocabulary::Event as ProbeEvent;

pub struct State {
    pub last: pv::Event,
    pub first: ProbeEvent,
}
