use patterns_macros::because;

pub fn clamp_upper<T: PartialOrd>(v: T, hi: T) -> T {
    match v > hi {
        true => hi,
        false => v,
    }
}
because!(clamp_upper, "a ceiling applied on its own, so a comparison never appears in spec or app");
