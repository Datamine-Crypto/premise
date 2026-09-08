use patterns::*;
use spec::State;

pub fn lent(state: &State) -> usize {
    size(&state.loans)
}
