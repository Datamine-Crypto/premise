use patterns::is_same;
use spec::*;

pub fn is_a(k: Kind) -> bool {
    is_same(k, Kind::A)
}
