use patterns_macros::because;

pub fn raise_by(v: u32, by: u32) -> u32 {
    v.saturating_add(by)
}
because!(raise_by, "addition that stops at the type ceiling rather than wrapping or panicking");

pub fn reduce_by(v: u32, by: u32) -> u32 {
    v.saturating_sub(by)
}
because!(reduce_by, "subtraction that stops at the type floor rather than wrapping or panicking");

pub fn settle_up(value: u32, floor: u32, roof: u32, step: u32) -> u32 {
    let moved = raise_by(value, step);
    let held = raise_by(moved, floor);
    reduce_by(held, roof)
}
because!(settle_up, "a settlement moving upward by a step, kept beside settle_down because the two directions are chosen by different callers and a direction parameter would put the choice in every call");

pub fn settle_down(value: u32, floor: u32, roof: u32, step: u32) -> u32 {
    let moved = reduce_by(value, step);
    let held = raise_by(moved, floor);
    reduce_by(held, roof)
}
because!(settle_down, "a settlement moving downward by a step, kept beside settle_up because the two directions are chosen by different callers and a direction parameter would put the choice in every call");
