use patterns::seq;
use spec::State;

pub fn lent(state: &State) -> usize {
    seq::size(&state.loans)
}
