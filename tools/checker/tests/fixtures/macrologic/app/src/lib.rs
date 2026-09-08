use patterns::{found, is_above};
use spec::State;

pub fn rich(state: &State, member: u32, floor: u32) -> bool {
    matches!(found(&state.members, member), Some(m) if is_above(m.owed, floor))
}
