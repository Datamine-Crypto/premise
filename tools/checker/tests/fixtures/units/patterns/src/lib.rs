use patterns_macros::because;

pub fn to_ms<T: Copy>(x: T) -> T {
    from_seconds(x)
}

pub fn to_kg<T: Copy>(y: T) -> T {
    from_pounds(y)
}
because!(to_ms, "a unit conversion named so the factor lives in the spec and never in a call site");
because!(to_kg, "a unit conversion named so the factor lives in the spec and never in a call site");
