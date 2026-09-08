pub trait Cap {
    fn ceiling(&self) -> u32;
}
pub struct Plan;
impl Cap for Plan {
    fn ceiling(&self) -> u32 {
        750
    }
}
