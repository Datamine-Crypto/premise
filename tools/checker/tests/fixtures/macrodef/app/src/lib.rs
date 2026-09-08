use spec::SLOT_CAP;

macro_rules! decide {
    ($v:expr) => {
        if $v > SLOT_CAP {
            SLOT_CAP
        } else {
            $v
        }
    };
}

pub fn k(v: u32) -> u32 {
    decide!(v)
}
