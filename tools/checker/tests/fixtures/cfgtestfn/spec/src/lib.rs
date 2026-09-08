use patterns::mul;

pub const TERM_DAYS: u32 = 5;

#[cfg(test)]
pub fn helper(n: u32) -> u32 {
    mul(n, TERM_DAYS)
}
