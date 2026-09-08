pub fn one<T: PartialOrd>(v: T, hi: T) -> T {
    if v > hi {
        hi
    } else {
        v
    }
}

pub fn r#two<T: PartialOrd>(r#v: T, r#hi: T) -> T {
    if r#v > r#hi {
        r#hi
    } else {
        r#v
    }
}
