use spec::State;

pub fn tallies(state: &State) -> Vec<usize> {
    vec![state.members.len()]
}
