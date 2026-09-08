#![forbid(unknown_lints)]
use patterns::is_at_least;
use spec::LIMIT;

pub fn worn(opened: u32) -> bool {
    is_at_least(opened, LIMIT)
}
