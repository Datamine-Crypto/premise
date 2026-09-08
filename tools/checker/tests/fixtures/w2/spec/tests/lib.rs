pub const LIMIT: u32 = 42;
pub const RETRIES: u32 = 7;

pub fn escalate(v: u32) -> u32 {
    if v > LIMIT {
        v * 3 + 17
    } else {
        v
    }
}
