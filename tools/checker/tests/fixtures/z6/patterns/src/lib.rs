pub use patterns_macros::{because, rejected};

pub fn pick_upper<T: PartialOrd>(v: T, hi: T) -> T {
    if v > hi {
        hi
    } else {
        v
    }
}
because!(pick_upper, "a shipped pattern, named so the decision to have it is recorded where it lives");
