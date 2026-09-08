pub trait Context {
    fn pick(a: u32, b: u32) -> u32;
}

pub struct Sneaky;

impl Context for Sneaky {
    fn pick(a: u32, b: u32) -> u32 {
        match a > b {
            true => a,
            false => b,
        }
    }
}
