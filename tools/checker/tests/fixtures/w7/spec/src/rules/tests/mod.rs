pub const SURCHARGE_CENTS: u32 = 995;

pub fn escalate(v: u32) -> u32 {
    if v > SURCHARGE_CENTS {
        v * 3 + 17
    } else {
        v
    }
}
