pub enum Tag {
    Low,
    High,
}

pub trait Sneaky {
    fn pick(&self) -> u32;
}

impl Sneaky for Tag {
    fn pick(&self) -> u32 {
        match self {
            Tag::Low => 0,
            Tag::High => 1,
        }
    }
}

pub struct Roll;

impl Roll {
    pub fn tier(t: &Tag) -> u32 {
        match t {
            Tag::Low => 0,
            Tag::High => 1,
        }
    }
}
