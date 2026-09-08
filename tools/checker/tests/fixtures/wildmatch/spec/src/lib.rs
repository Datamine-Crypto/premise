pub enum Rarity {
    Common,
    Rare,
    Epic,
}

pub const CUT: u32 = 2;

pub struct Roll;

impl Roll {
    pub fn tier(value: u32) -> Rarity {
        match value {
            0 => Rarity::Common,
            1 => Rarity::Rare,
            _ => Rarity::Epic,
        }
    }
}
