pub fn pick_high<T: PartialOrd>(v: T, hi: T) -> T {
    if v > hi {
        hi
    } else {
        v
    }
}

pub struct Helper;

impl Helper {
    pub fn hidden<T: PartialOrd>(v: T, hi: T) -> T {
        if v > hi {
            hi
        } else {
            v
        }
    }
}
