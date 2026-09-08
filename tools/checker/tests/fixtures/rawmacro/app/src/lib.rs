use spec::CAP;

pub fn k(v: u32) -> u32 {
    patterns::decide!(v, CAP)
}
