pub use patterns_macros::{because, fact, rejected};

pub fn pick_upper<T: PartialOrd>(v: T, hi: T) -> T {
    if v > hi {
        hi
    } else {
        v
    }
}
