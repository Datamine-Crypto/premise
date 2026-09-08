use spec::LIMIT;

pub fn cap(hp: u32, dmg: u32) -> u32 {
    u32::wrapping_sub(hp, dmg)
}
