pub fn settle_up<T: PartialOrd>(v: T, floor: T, roof: T) -> T {
    clamp_range(raise_by(v, floor), floor, roof)
}

pub fn settle_down<T: PartialOrd>(v: T, floor: T, roof: T) -> T {
    clamp_range(reduce_by(v, floor), floor, roof)
}
