pub struct Machine;

impl Machine {
    #[allow(non_snake_case)]
    pub fn  Spin(v: u32, cap: u32) -> u32 {
        patterns::clamp_upper(v, cap)
    }
}

pub fn a1(v: u32) -> u32 {
    Machine::Spin(v, 0)
}
