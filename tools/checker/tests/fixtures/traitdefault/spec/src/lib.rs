use patterns::mul;

pub trait Pricing {
    fn price(&self, days: u32) -> u32 {
        mul(days, RATE)
    }
}
pub const RATE: u32 = 15;
