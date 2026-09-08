pub fn cap_then_floor_q<T: PartialOrd>(v: T, lo: T, hi: T) -> T {
    clamp_lower(clamp_upper(v, hi), lo)
}

pub fn floor_then_cap_q<T: PartialOrd>(v: T, lo: T, hi: T) -> T {
    let out = clamp_upper(clamp_lower(v, hi), lo);
    out
}
