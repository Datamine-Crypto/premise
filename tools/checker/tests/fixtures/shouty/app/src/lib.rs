pub struct Machine;

impl Machine {
    #[allow(non_snake_case)]
    pub fn Spin(v: u32, cap: u32) -> u32 {
        patterns::clamp_upper(v, cap)
    }
}

pub enum Kind {
    Wide,
}

pub fn a2(v: u32) -> u32 {
    Machine::Spin(v, 0)
}

pub fn a4(k: Kind) -> Kind {
    k
}

pub fn a5() -> Kind {
    Kind::Wide
}
