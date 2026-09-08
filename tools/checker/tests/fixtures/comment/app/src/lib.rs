use patterns::pick_upper;
use spec::LIMIT;

// this explains the function
pub fn cap(v: u32) -> u32 {
    pick_upper(v, LIMIT)
}
