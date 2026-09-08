use patterns::is_same;
use spec::*;
use std::time::Duration;

pub fn is_a(k: Kind) -> bool {
    is_same(k, Kind::A)
}

pub fn longest() -> Duration {
    Duration::MAX
}
