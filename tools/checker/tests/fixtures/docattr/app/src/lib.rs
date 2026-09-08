use spec::LIMIT;

#[doc = "this explains the function"]
pub fn cap(v: u32) -> u32 {
    pick_upper(v, LIMIT)
}
