pub fn pick_a<T: PartialOrd>(v: T, hi: T) -> T {
    if v > hi {
        hi
    } else {
        v
    }
}

pub fn pick_b<T: Ord>(v: T, hi: T) -> T {
    if v > hi {
        hi
    } else {
        v
    }
}
