use patterns::Bounded as Context;

pub mod fake {
    pub trait Context {
        fn route(&self, tag: u32) -> u32;
    }
}

pub struct NotAReducer;

impl fake::Context for NotAReducer {
    fn route(&self, tag: u32) -> u32 {
        match tag {
            0 => 1,
            1 => 0,
            _ => tag,
        }
    }
}

#[allow(non_snake_case)]
pub fn Context(v: u32) -> u32 {
    v
}
