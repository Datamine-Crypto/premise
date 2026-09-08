use patterns_macros::because;

pub fn clamp_upper<T: PartialOrd>(v: T, hi: T) -> T {
    if v > hi {
        hi
    } else {
        v
    }
}

pub fn clamp_lower<T: PartialOrd>(v: T, lo: T) -> T {
    if v < lo {
        lo
    } else {
        v
    }
}

pub fn clamp_range<T: PartialOrd>(v: T, lo: T, hi: T) -> T {
    clamp_upper(clamp_lower(v, lo), hi)
}
because!(clamp_upper, "a ceiling applied on its own, because a value with a maximum and no minimum is common enough to deserve its own name");
because!(clamp_lower, "a floor applied on its own, because a value with a minimum and no maximum is as common as the other way round");
because!(clamp_range, "both bounds at once, which is not clamp_lower composed with clamp_upper for the caller because writing it twice invites writing it in the wrong order");
