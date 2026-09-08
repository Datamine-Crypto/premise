use patterns::raise_by;

pub type State = (u32, u32);
pub struct Meter;
pub trait Rule {
    fn step(state: State) -> State;
}
impl Rule for Meter {
    fn step(state: State) -> State {
        (raise_by(state.0, 1), state.1)
    }
}
