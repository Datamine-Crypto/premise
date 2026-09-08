pub const REAL_CAP: u32 = 250;

pub fn ship(v: u32) -> u32 {
    if v > REAL_CAP {
        REAL_CAP
    } else {
        v * 3 + 17
    }
}

macro_rules! anything {
    () => {
        0
    };
}
