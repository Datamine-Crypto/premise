use spec::LIMIT;

pub fn cap(v: u32) -> u32 {
    v.min(LIMIT)
}
