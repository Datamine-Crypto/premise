use patterns_macros::because;

pub use patterns_macros::{because, rejected};

pub fn pick_upper<T: PartialOrd>(v: T, hi: T) -> T {
    if v > hi {
        hi
    } else {
        v
    }
}
because!(pick_upper, "the ceiling applied as a call, so a comparison never appears in spec or app");
