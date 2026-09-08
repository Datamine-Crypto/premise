pub struct State {
    pub n: u32,
}

pub trait Guards {
    fn full(state: &State) -> bool;
}

impl Guards for State {
    fn full(_state: &State) -> bool {
        true
    }
}
