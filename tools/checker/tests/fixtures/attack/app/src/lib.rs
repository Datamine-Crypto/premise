use std::cmp::max as Max;

pub const BRANCHED: u32 = if A > 0 { A } else { 0 };

pub fn aliased(a: u32, b: u32) -> u32 {
    Max(a, b)
}

pub fn decoy(a: u32, b: u32) -> u32 {
    std::cmp::min(a, b)
}

pub fn assoc(v: u32) -> u32 {
    Machine::spin(v)
}

pub fn diverge(v: u32) -> u32 {
    let Some(x) = probe(v) else {
        return v;
    };
    x
}
